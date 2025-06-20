use macro_types::breadcrumbs::{BreadcrumbComponentValue, BreadcrumbPathListValue, SystemBreadcrumbComponent, SystemBreadcrumbPath};
use macro_types::environment::MacroIO;
use macro_types::project::ProjectContext;


// ————————————————————————————————————————————————————————————————————————————
// API
// ————————————————————————————————————————————————————————————————————————————

pub fn to_breadcrumb_value_path(system_breadcrumb_path: &SystemBreadcrumbPath, project_context: &ProjectContext) -> MacroIO<BreadcrumbPathListValue> {
    let (components, deps ) = system_breadcrumb_path.components
        .iter()
        .map(|SystemBreadcrumbComponent { source, title }| {
            let host_path = system_breadcrumb_path.file_input.resolved_public_path(project_context);
            let target_path = source.resolved_public_path(project_context);
            let relative = pathdiff::diff_paths(&target_path, host_path.parent().unwrap()).unwrap();
            let relative = relative.to_str().unwrap();
            let dependency = system_breadcrumb_path.file_input.with_dependency_relation(relative);
            let result = BreadcrumbComponentValue {
                href: format!("noop://{relative}"),
                title: title.to_owned(),
            };
            (result, dependency)
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();
        // .collect::<Vec<_>>();
    MacroIO::wrap(BreadcrumbPathListValue(components)).and_modify_context(|_| {
        // ctx.dependencies.extend(deps);
        let _ = deps;
    })
}

// pub fn to_breadcrumb_value_path(system_breadcrumb_path: &SystemBreadcrumbPath, project_context: &ProjectContext) -> MacroIO<BreadcrumbPathListValue> {
//     let components = system_breadcrumb_path.components
//         .iter()
//         .map(|SystemBreadcrumbComponent { source, title }| {
//             let host_path = system_breadcrumb_path.file_input.resolved_public_path(project_context);
//             let target_path = source.resolved_public_path(project_context);
//             let relative = pathdiff::diff_paths(&target_path, host_path.parent().unwrap()).unwrap();
//             let relative = relative.to_str().unwrap();
//             let dependency = system_breadcrumb_path.file_input.with_dependency_relation(relative);
//             let result = BreadcrumbComponentValue {
//                 href: format!("/noop://{relative}"),
//                 title: title.to_owned(),
//             };
//             result
//         })
//         .collect::<Vec<_>>();
//         // .collect::<Vec<_>>();
//     MacroIO::wrap(BreadcrumbPathListValue(components))
// }

