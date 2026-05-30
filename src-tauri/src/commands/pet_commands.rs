use tauri::command;
use crate::modules::pet::PetStatus;
use crate::services::pet_service::PetService;

#[command]
pub fn feed_pet() -> Result<PetStatus, String> {
    let mut pet_service = PetService::instance();
    pet_service.feed();
    Ok(pet_service.get_status())
}

#[command]
pub fn play_with_pet() -> Result<PetStatus, String> {
    let mut pet_service = PetService::instance();
    pet_service.play();
    Ok(pet_service.get_status())
}

#[command]
pub fn pet_status() -> Result<PetStatus, String> {
    let pet_service = PetService::instance();
    Ok(pet_service.get_status())
}

#[command]
pub fn get_pet_status() -> Result<PetStatus, String> {
    let pet_service = PetService::instance();
    Ok(pet_service.get_status())
}