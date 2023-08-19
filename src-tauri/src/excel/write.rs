use anyhow::Result;

use rust_xlsxwriter::*;

use crate::models::{user::User, group::Group};

pub fn write_excel(path: String, user: Option<User>, group: Option<Group>) -> Result<()> {

    let mut workbook = Workbook::new();

    if user.is_some() {
        let worksheet = workbook.add_worksheet().set_name("Users")?;
        worksheet.write_row(0, 0, ["ID", "Display Name", "Name", "Type", "Location Type"])?;

        let user = user.unwrap();
    
        user.results.iter().enumerate().for_each(|(i, user)| {
            worksheet.write_row(i as u32 + 1, 0, [
                user.id.to_string(),
                user.display_name.to_string(),
                user.name.to_string(),
                user.type_field.to_string(),
                user.location_type.to_string(),
            ]).unwrap();
        });
    };
    
    if group.is_some() {
        let worksheet = workbook.add_worksheet().set_name("Groups")?;
        worksheet.write_row(0, 0, ["ID", "Display Name", "Name", "Type", "Location Type", "Managed by", "Site"])?;

        let group = group.unwrap();
    
        group.results.iter().enumerate().for_each(|(i, group)| {
            worksheet.write_row(i as u32 + 1, 0, [
                group.id.to_string(),
                group.display_name.to_string(),
                group.name.to_string(),
                group.type_field.to_string(),
                group.location_type.to_string(),
                group.managed_by.to_string(),
                group.site.clone().unwrap_or("".to_string()),
            ]).unwrap();
        });
    }


    workbook.save(path)?;

    Ok(())
}