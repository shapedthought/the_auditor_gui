use crate::models::{audit::AuditItem, user::User, group::Group};
use calamine::{Reader, open_workbook, Xlsx};
use anyhow::{Result, Error};

pub fn read_excel(path: String, user: Option<User>, group: Option<Group>) -> Result<Vec<AuditItem>> {
    let mut excel: Xlsx<_> = open_workbook(path)?;

    let sheet_type = if user.is_some() {
        "Users"
    } else if group.is_some() {
        "Groups"
    } else {
        Err(Error::msg("No sheet type found"))?
    };

    let workbook = excel.worksheet_range(sheet_type).unwrap()?; 

    let id_strings = workbook.rows().skip(1).map(|row| {
        let name = row[0].get_string().unwrap();
        name.to_string()
    }).collect::<Vec<String>>();

    if user.is_some() {
        let user = user.unwrap();
        let mut audit_items = Vec::new();
        user.results.iter().for_each(|user| {
            if id_strings.contains(&user.id) {
                audit_items.push(AuditItem::from(user.clone()));
            }
        });
        Ok(audit_items)
    } else if group.is_some() {
        let group = group.unwrap();
        let mut audit_items = Vec::new();
        group.results.iter().for_each(|group| {
            if id_strings.contains(&group.id) {
                audit_items.push(AuditItem::from(group.clone()));
            }
        });
        Ok(audit_items)
    } else {
        Err(Error::msg("No sheet type found"))?
    }
}