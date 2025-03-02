use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use tracing::{debug, info};

// iOS-specific imports
#[cfg(target_os = "ios")]
use objc::{class, msg_send, sel, sel_impl};
#[cfg(target_os = "ios")]
use metal::{Device, CommandQueue};

// Required by iOS for FFI
#[no_mangle]
pub extern "C" fn ios_main() {
    info!("Rust AR iOS app starting");
    initialize_ar_session();
}

// Globals to store AR state
static mut AR_SESSION: Option<Arc<Mutex<ARSession>>> = None;

// Simple struct to hold AR state
struct ARSession {
    initialized: bool,
    camera_position: [f32; 3],
    detected_planes: Vec<ARPlane>,
    virtual_objects: Vec<ARObject>,
}

// Structure for detected AR planes
struct ARPlane {
    id: String,
    center: [f32; 3],
    extent: [f32; 2],
    normal: [f32; 3],
}

// Structure for virtual objects in AR
struct ARObject {
    id: String,
    position: [f32; 3],
    rotation: [f32; 4], // Quaternion
    object_type: ARObjectType,
}

// Types of AR objects
enum ARObjectType {
    Cube,
    Sphere,
    Custom(String),
}

// Initialize the AR session
fn initialize_ar_session() {
    let session = ARSession {
        initialized: true,
        camera_position: [0.0, 0.0, 0.0],
        detected_planes: Vec::new(),
        virtual_objects: Vec::new(),
    };
    
    // Store in global state
    unsafe {
        AR_SESSION = Some(Arc::new(Mutex::new(session)));
    }
    
    info!("AR session initialized from Rust");
}

// Update the AR camera position
#[no_mangle]
pub extern "C" fn update_camera_position(x: f32, y: f32, z: f32) {
    unsafe {
        if let Some(session) = &AR_SESSION {
            if let Ok(mut session_lock) = session.lock() {
                session_lock.camera_position = [x, y, z];
            }
        }
    }
}

// Add a detected plane
#[no_mangle]
pub extern "C" fn add_detected_plane(
    id_ptr: *const libc::c_char,
    center_x: f32, center_y: f32, center_z: f32,
    width: f32, height: f32,
    normal_x: f32, normal_y: f32, normal_z: f32
) {
    unsafe {
        if let Some(session) = &AR_SESSION {
            if let Ok(mut session_lock) = session.lock() {
                // Convert C string to Rust string
                let id = if !id_ptr.is_null() {
                    let c_str = std::ffi::CStr::from_ptr(id_ptr);
                    c_str.to_string_lossy().into_owned()
                } else {
                    format!("plane_{}", session_lock.detected_planes.len())
                };
                
                // Create new plane
                let plane = ARPlane {
                    id,
                    center: [center_x, center_y, center_z],
                    extent: [width, height],
                    normal: [normal_x, normal_y, normal_z],
                };
                
                // Add to session
                session_lock.detected_planes.push(plane);
                
                info!("Added plane: center=[{}, {}, {}], extent=[{}, {}]", 
                    center_x, center_y, center_z, width, height);
            }
        }
    }
}

// Place a virtual object in AR space
#[no_mangle]
pub extern "C" fn place_virtual_object(
    object_type: i32,
    pos_x: f32, pos_y: f32, pos_z: f32,
    rot_x: f32, rot_y: f32, rot_z: f32, rot_w: f32
) -> i32 {
    unsafe {
        if let Some(session) = &AR_SESSION {
            if let Ok(mut session_lock) = session.lock() {
                // Determine object type
                let object_type = match object_type {
                    0 => ARObjectType::Cube,
                    1 => ARObjectType::Sphere,
                    _ => ARObjectType::Custom(format!("custom_{}", object_type)),
                };
                
                // Create new object
                let object = ARObject {
                    id: format!("object_{}", session_lock.virtual_objects.len()),
                    position: [pos_x, pos_y, pos_z],
                    rotation: [rot_x, rot_y, rot_z, rot_w],
                    object_type,
                };
                
                // Add to session
                let object_id = session_lock.virtual_objects.len() as i32;
                session_lock.virtual_objects.push(object);
                
                info!("Placed object {} at position [{}, {}, {}]", 
                    object_id, pos_x, pos_y, pos_z);
                    
                return object_id;
            }
        }
    }
    
    // Return -1 if failed
    -1
}

// Remove a virtual object
#[no_mangle]
pub extern "C" fn remove_virtual_object(object_id: i32) -> bool {
    unsafe {
        if let Some(session) = &AR_SESSION {
            if let Ok(mut session_lock) = session.lock() {
                if object_id >= 0 && (object_id as usize) < session_lock.virtual_objects.len() {
                    // Remove the object (this shifts array indices, but Swift will maintain its own mapping)
                    session_lock.virtual_objects.remove(object_id as usize);
                    println!("Removed object {}", object_id);
                    return true;
                }
            }
        }
    }
    
    false
}

// Get statistics about the AR session (for debugging)
#[no_mangle]
pub extern "C" fn get_session_stats(
    num_planes: *mut i32,
    num_objects: *mut i32
) {
    unsafe {
        if let Some(session) = &AR_SESSION {
            if let Ok(session_lock) = session.lock() {
                if !num_planes.is_null() {
                    *num_planes = session_lock.detected_planes.len() as i32;
                }
                
                if !num_objects.is_null() {
                    *num_objects = session_lock.virtual_objects.len() as i32;
                }
            }
        }
    }
}

// iOS-specific Metal setup code
#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn setup_metal_context(device_ptr: *mut std::ffi::c_void) -> bool {
    unsafe {
        if device_ptr.is_null() {
            return false;
        }
        
        // Convert the raw pointer to a Metal device
        let device_obj = device_ptr as *mut objc::runtime::Object;
        
        println!("Received Metal device from Swift");
        
        // In a real app, you would store this device for later use
        
        true
    }
}

// For testing on non-iOS platforms
#[cfg(not(target_os = "ios"))]
pub fn main() {
    info!("This AR app is designed for iOS, but you're running it on another platform.");
    info!("Building and testing functions...");
    
    initialize_ar_session();
    update_camera_position(1.0, 2.0, 3.0);
    
    // Add a test plane
    add_detected_plane(
        std::ptr::null(),  // Null ID for testing
        0.0, 0.0, -1.0,    // center
        1.0, 1.0,          // extent
        0.0, 1.0, 0.0      // normal (up)
    );
    
    // Place a test object
    let object_id = place_virtual_object(
        0,                  // Cube
        0.0, 0.5, -1.0,     // position
        0.0, 0.0, 0.0, 1.0  // rotation (identity quaternion)
    );
    
    // Get stats
    let mut num_planes = 0;
    let mut num_objects = 0;
    get_session_stats(&mut num_planes, &mut num_objects);
    
    info!("Stats: {} planes, {} objects", num_planes, num_objects);
    
    // Remove the object
    let removed = remove_virtual_object(object_id);
    info!("Object removed: {}", removed);
}