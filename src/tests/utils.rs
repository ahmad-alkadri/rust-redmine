use std::env;

#[allow(dead_code)]
pub struct UrlApik {
    pub url: String,
    pub apik: String,
}

#[allow(dead_code)]
impl UrlApik {
    pub fn new() -> UrlApik {
        UrlApik {
            url: env::var("REDMINE_URL").unwrap_or_else(|_| "".to_string()),
            apik: env::var("REDMINE_API_KEY").unwrap_or_else(|_| "".to_string()),
        }
    }

    pub fn any_empty(&self) -> bool {
        self.url.is_empty() || self.apik.is_empty()
    }
}

#[cfg(test)]
pub fn example_issue_filter(project_id: String) -> crate::fields::issues::IssueFilter {
    use crate::fields::issues::IssueFilter;
    return IssueFilter {
        project_id: Some(project_id),
        subproject_id: None,
        tracker_id: None,
        status_id: None,
        assigned_to_id: None,
        updated_on: None,
        extra_filters: None,
    };
}

#[cfg(test)]
pub fn example_issue(subject: String, project_id: String) -> crate::fields::issues::Issue {
    use crate::fields::issues::Issue;
    return Issue {
        id: None,
        subject: Some(subject),
        description: Some("".to_string()),
        project_id: Some(project_id),
        project: None,
        tracker_id: None,
        tracker: None,
        parent_issue_id: None,
        parent: None,
        status_id: None,
        status: None,
        priority_id: None,
        priority: None,
        author: None,
        fixed_version: None,
        assigned_to: None,
        assigned_to_id: None,
        category: None,
        category_id: None,
        notes: None,
        status_date: None,
        created_on: None,
        updated_on: None,
        start_date: None,
        due_date: None,
        closed_on: None,
        custom_fields: None,
        uploads: None,
        done_ratio: None,
        estimated_hours: None,
        journals: None,
    };
}

#[cfg(test)]
pub fn example_project(
    project_id: String,
    project_name: String,
) -> crate::fields::projects::Project {
    return crate::fields::projects::Project {
        name: Some(project_name),
        identifier: project_id.clone(),
        is_public: Some(false),
        description: Some(format!(
            "This is an example project with id: {:?}",
            &project_id
        )),
        id: None,
        parent: None,
        parent_id: None,
        inherit_members: None,
        tracker_ids: None,
        created_on: None,
        updated_on: None,
        enabled_module_names: None,
        custom_field_values: None,
    };
}
