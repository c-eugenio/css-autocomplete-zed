use std::collections::HashSet;

/// Returns all Bootstrap 5.3 class names (components + generated utilities).
pub fn bootstrap5_classes() -> HashSet<String> {
    let mut s: HashSet<String> = STATIC.iter().map(|c| c.to_string()).collect();

    let bp = ["", "sm-", "md-", "lg-", "xl-", "xxl-"];

    // ── Display ──────────────────────────────────────────────────────────────
    for b in bp {
        for v in ["none","inline","inline-block","block","grid","inline-grid",
                  "table","table-row","table-cell","flex","inline-flex"] {
            s.insert(format!("d-{b}{v}"));
        }
    }
    for v in ["none","inline","inline-block","block","grid","inline-grid",
              "table","table-row","table-cell","flex","inline-flex"] {
        s.insert(format!("d-print-{v}"));
    }

    // ── Spacing m / p ────────────────────────────────────────────────────────
    for prop in ["m", "p"] {
        for dir in ["", "t", "b", "s", "e", "x", "y"] {
            for b in bp {
                for v in ["0","1","2","3","4","5","auto"] {
                    s.insert(format!("{prop}{dir}-{b}{v}"));
                }
            }
        }
    }
    // Negative margins
    for dir in ["", "t", "b", "s", "e", "x", "y"] {
        for b in bp {
            for v in ["1","2","3","4","5"] {
                s.insert(format!("m{dir}-{b}n{v}"));
            }
        }
    }

    // ── Gap ──────────────────────────────────────────────────────────────────
    for b in bp {
        for v in ["0","1","2","3","4","5"] {
            s.insert(format!("gap-{b}{v}"));
            s.insert(format!("row-gap-{b}{v}"));
            s.insert(format!("column-gap-{b}{v}"));
            s.insert(format!("g-{b}{v}"));
            s.insert(format!("gx-{b}{v}"));
            s.insert(format!("gy-{b}{v}"));
        }
    }

    // ── Flex ─────────────────────────────────────────────────────────────────
    for b in bp {
        for v in ["row","row-reverse","column","column-reverse"] {
            s.insert(format!("flex-{b}{v}"));
        }
        for v in ["wrap","wrap-reverse","nowrap"] {
            s.insert(format!("flex-{b}{v}"));
        }
        s.insert(format!("flex-{b}fill"));
        for v in ["0","1"] {
            s.insert(format!("flex-{b}grow-{v}"));
            s.insert(format!("flex-{b}shrink-{v}"));
        }
        for v in ["start","end","center","baseline","stretch"] {
            s.insert(format!("align-items-{b}{v}"));
        }
        for v in ["start","end","center","baseline","stretch","auto"] {
            s.insert(format!("align-self-{b}{v}"));
        }
        for v in ["start","end","center","between","around","evenly"] {
            s.insert(format!("justify-content-{b}{v}"));
        }
        for v in ["0","1","2","3","4","5","first","last"] {
            s.insert(format!("order-{b}{v}"));
        }
    }

    // ── Float ────────────────────────────────────────────────────────────────
    for b in bp {
        for v in ["start","end","none"] {
            s.insert(format!("float-{b}{v}"));
        }
    }

    // ── Object fit ───────────────────────────────────────────────────────────
    for b in bp {
        for v in ["contain","cover","fill","scale","none"] {
            s.insert(format!("object-fit-{b}{v}"));
        }
    }

    // ── Text ─────────────────────────────────────────────────────────────────
    // Alignment (responsive)
    for b in bp {
        for v in ["start","center","end"] {
            s.insert(format!("text-{b}{v}"));
        }
    }
    // Other text utilities
    for v in ["wrap","nowrap","break","truncate","lowercase","uppercase","capitalize",
              "muted","white","black","reset",
              "primary","secondary","success","danger","warning","info","light","dark",
              "primary-emphasis","secondary-emphasis","success-emphasis",
              "danger-emphasis","warning-emphasis","info-emphasis",
              "light-emphasis","dark-emphasis",
              "body","body-secondary","body-tertiary"] {
        s.insert(format!("text-{v}"));
    }
    for v in ["none","underline","line-through"] {
        s.insert(format!("text-decoration-{v}"));
    }
    for v in ["25","50","75","100"] {
        s.insert(format!("text-opacity-{v}"));
    }

    // Font weight / style / size / line-height
    for v in ["bold","bolder","semibold","normal","light","lighter"] {
        s.insert(format!("fw-{v}"));
    }
    for v in ["italic","normal"] {
        s.insert(format!("fst-{v}"));
    }
    for v in ["1","2","3","4","5","6"] {
        s.insert(format!("fs-{v}"));
    }
    for v in ["1","sm","base","lg"] {
        s.insert(format!("lh-{v}"));
    }

    // ── Background ───────────────────────────────────────────────────────────
    for v in ["primary","secondary","success","danger","warning","info","light","dark",
              "white","black","transparent","body","body-secondary","body-tertiary",
              "primary-subtle","secondary-subtle","success-subtle","danger-subtle",
              "warning-subtle","info-subtle","light-subtle","dark-subtle","gradient"] {
        s.insert(format!("bg-{v}"));
    }
    for v in ["10","25","50","75","100"] {
        s.insert(format!("bg-opacity-{v}"));
    }

    // ── Border ───────────────────────────────────────────────────────────────
    for dir in ["","-top","-end","-bottom","-start"] {
        s.insert(format!("border{dir}"));
        s.insert(format!("border{dir}-0"));
    }
    for v in ["primary","secondary","success","danger","warning","info","light","dark",
              "white","black","primary-subtle","secondary-subtle","success-subtle",
              "danger-subtle","warning-subtle","info-subtle","light-subtle","dark-subtle"] {
        s.insert(format!("border-{v}"));
    }
    for v in ["1","2","3","4","5"] {
        s.insert(format!("border-{v}"));
    }
    for v in ["10","25","50","75","100"] {
        s.insert(format!("border-opacity-{v}"));
    }

    // ── Border radius ────────────────────────────────────────────────────────
    s.insert("rounded".into());
    s.insert("rounded-circle".into());
    s.insert("rounded-pill".into());
    for v in ["0","1","2","3","4","5"] {
        s.insert(format!("rounded-{v}"));
    }
    for dir in ["top","end","bottom","start",
                "top-start","top-end","bottom-start","bottom-end"] {
        s.insert(format!("rounded-{dir}"));
        for v in ["0","1","2","3","4","5"] {
            s.insert(format!("rounded-{dir}-{v}"));
        }
    }

    // ── Sizing ───────────────────────────────────────────────────────────────
    for v in ["25","50","75","100","auto"] {
        s.insert(format!("w-{v}"));
        s.insert(format!("h-{v}"));
    }
    s.insert("mw-100".into()); s.insert("mh-100".into());
    s.insert("min-vw-100".into()); s.insert("min-vh-100".into());
    s.insert("vw-100".into()); s.insert("vh-100".into());

    // ── Position ─────────────────────────────────────────────────────────────
    for v in ["static","relative","absolute","fixed","sticky"] {
        s.insert(format!("position-{v}"));
    }
    for v in ["0","50","100"] {
        s.insert(format!("top-{v}"));
        s.insert(format!("bottom-{v}"));
        s.insert(format!("start-{v}"));
        s.insert(format!("end-{v}"));
    }
    s.insert("translate-middle".into());
    s.insert("translate-middle-x".into());
    s.insert("translate-middle-y".into());

    // ── Shadow ───────────────────────────────────────────────────────────────
    for v in ["","-sm","-lg","-none"] {
        s.insert(format!("shadow{v}"));
    }

    // ── Opacity ──────────────────────────────────────────────────────────────
    for v in ["0","25","50","75","100"] {
        s.insert(format!("opacity-{v}"));
    }

    // ── Overflow ─────────────────────────────────────────────────────────────
    for ax in ["","x-","y-"] {
        for v in ["auto","hidden","visible","scroll"] {
            s.insert(format!("overflow-{ax}{v}"));
        }
    }

    // ── Visibility ───────────────────────────────────────────────────────────
    s.insert("visible".into());
    s.insert("invisible".into());
    s.insert("visually-hidden".into());
    s.insert("visually-hidden-focusable".into());

    // ── Z-index ──────────────────────────────────────────────────────────────
    for v in ["n1","0","1","2","3"] {
        s.insert(format!("z-{v}"));
    }

    // ── Misc utilities ───────────────────────────────────────────────────────
    for v in ["all","auto","none"] {
        s.insert(format!("user-select-{v}"));
    }
    s.insert("pe-none".into()); s.insert("pe-auto".into());
    for v in ["baseline","top","middle","bottom","text-top","text-bottom"] {
        s.insert(format!("align-{v}"));
    }

    // ── Link utilities ───────────────────────────────────────────────────────
    for c in ["primary","secondary","success","danger","warning","info","light","dark"] {
        s.insert(format!("link-{c}"));
        s.insert(format!("link-{c}-emphasis"));
        s.insert(format!("link-underline-{c}"));
    }
    s.insert("stretched-link".into());
    s.insert("link-underline".into());
    for v in ["10","25","50","75","100"] { s.insert(format!("link-opacity-{v}")); }
    for v in ["1","2","3"] { s.insert(format!("link-offset-{v}")); }
    s.insert("icon-link".into());

    // ── Color helpers (text-bg-*) ─────────────────────────────────────────────
    for c in ["primary","secondary","success","danger","warning","info","light","dark"] {
        s.insert(format!("text-bg-{c}"));
    }

    // ── Ratio ────────────────────────────────────────────────────────────────
    s.insert("ratio".into());
    for v in ["1x1","4x3","16x9","21x9"] { s.insert(format!("ratio-{v}")); }

    s
}

// ── Static component & layout classes ────────────────────────────────────────
static STATIC: &[&str] = &[
    // Grid
    "col","col-auto","col-1","col-2","col-3","col-4","col-5","col-6",
    "col-7","col-8","col-9","col-10","col-11","col-12",
    "col-sm","col-sm-auto","col-sm-1","col-sm-2","col-sm-3","col-sm-4","col-sm-5","col-sm-6",
    "col-sm-7","col-sm-8","col-sm-9","col-sm-10","col-sm-11","col-sm-12",
    "col-md","col-md-auto","col-md-1","col-md-2","col-md-3","col-md-4","col-md-5","col-md-6",
    "col-md-7","col-md-8","col-md-9","col-md-10","col-md-11","col-md-12",
    "col-lg","col-lg-auto","col-lg-1","col-lg-2","col-lg-3","col-lg-4","col-lg-5","col-lg-6",
    "col-lg-7","col-lg-8","col-lg-9","col-lg-10","col-lg-11","col-lg-12",
    "col-xl","col-xl-auto","col-xl-1","col-xl-2","col-xl-3","col-xl-4","col-xl-5","col-xl-6",
    "col-xl-7","col-xl-8","col-xl-9","col-xl-10","col-xl-11","col-xl-12",
    "col-xxl","col-xxl-auto","col-xxl-1","col-xxl-2","col-xxl-3","col-xxl-4","col-xxl-5","col-xxl-6",
    "col-xxl-7","col-xxl-8","col-xxl-9","col-xxl-10","col-xxl-11","col-xxl-12",
    "col-form-label","col-form-label-lg","col-form-label-sm",
    "row","row-cols-1","row-cols-2","row-cols-3","row-cols-4","row-cols-5","row-cols-6","row-cols-auto",
    "row-cols-sm-1","row-cols-sm-2","row-cols-sm-3","row-cols-sm-4","row-cols-sm-5","row-cols-sm-6","row-cols-sm-auto",
    "row-cols-md-1","row-cols-md-2","row-cols-md-3","row-cols-md-4","row-cols-md-5","row-cols-md-6","row-cols-md-auto",
    "row-cols-lg-1","row-cols-lg-2","row-cols-lg-3","row-cols-lg-4","row-cols-lg-5","row-cols-lg-6","row-cols-lg-auto",
    "row-cols-xl-1","row-cols-xl-2","row-cols-xl-3","row-cols-xl-4","row-cols-xl-5","row-cols-xl-6","row-cols-xl-auto",
    "row-cols-xxl-1","row-cols-xxl-2","row-cols-xxl-3","row-cols-xxl-4","row-cols-xxl-5","row-cols-xxl-6","row-cols-xxl-auto",
    "container","container-fluid","container-sm","container-md","container-lg","container-xl","container-xxl",
    // Offsets
    "offset-1","offset-2","offset-3","offset-4","offset-5","offset-6",
    "offset-7","offset-8","offset-9","offset-10","offset-11",
    "offset-sm-0","offset-sm-1","offset-sm-2","offset-sm-3","offset-sm-4","offset-sm-5",
    "offset-sm-6","offset-sm-7","offset-sm-8","offset-sm-9","offset-sm-10","offset-sm-11",
    "offset-md-0","offset-md-1","offset-md-2","offset-md-3","offset-md-4","offset-md-5",
    "offset-md-6","offset-md-7","offset-md-8","offset-md-9","offset-md-10","offset-md-11",
    "offset-lg-0","offset-lg-1","offset-lg-2","offset-lg-3","offset-lg-4","offset-lg-5",
    "offset-lg-6","offset-lg-7","offset-lg-8","offset-lg-9","offset-lg-10","offset-lg-11",
    "offset-xl-0","offset-xl-1","offset-xl-2","offset-xl-3","offset-xl-4","offset-xl-5",
    "offset-xl-6","offset-xl-7","offset-xl-8","offset-xl-9","offset-xl-10","offset-xl-11",
    "offset-xxl-0","offset-xxl-1","offset-xxl-2","offset-xxl-3","offset-xxl-4","offset-xxl-5",
    "offset-xxl-6","offset-xxl-7","offset-xxl-8","offset-xxl-9","offset-xxl-10","offset-xxl-11",
    // State
    "active","disabled","show","fade","collapse","collapsing","collapse-horizontal",
    "is-valid","is-invalid","was-validated",
    // Typography helpers
    "h1","h2","h3","h4","h5","h6","lead","mark","small","initialism","display-1",
    "display-2","display-3","display-4","display-5","display-6",
    "list-unstyled","list-inline","list-inline-item",
    "img-fluid","img-thumbnail","figure","figure-img","figure-caption",
    // Buttons
    "btn","btn-primary","btn-secondary","btn-success","btn-danger","btn-warning","btn-info",
    "btn-light","btn-dark","btn-link",
    "btn-outline-primary","btn-outline-secondary","btn-outline-success","btn-outline-danger",
    "btn-outline-warning","btn-outline-info","btn-outline-light","btn-outline-dark",
    "btn-lg","btn-sm","btn-close","btn-close-white","btn-check",
    "btn-group","btn-group-lg","btn-group-sm","btn-group-vertical","btn-toolbar",
    // Accordion
    "accordion","accordion-item","accordion-header","accordion-button","accordion-body","accordion-flush",
    // Alert
    "alert","alert-primary","alert-secondary","alert-success","alert-danger","alert-warning",
    "alert-info","alert-light","alert-dark","alert-dismissible","alert-link","alert-heading",
    // Badge
    "badge",
    // Breadcrumb
    "breadcrumb","breadcrumb-item","breadcrumb-divider",
    // Card
    "card","card-body","card-title","card-subtitle","card-text","card-link",
    "card-header","card-footer","card-img","card-img-top","card-img-bottom","card-img-overlay",
    "card-group","card-header-tabs","card-header-pills",
    // Carousel
    "carousel","carousel-inner","carousel-item","carousel-fade","carousel-dark",
    "carousel-control-prev","carousel-control-next","carousel-control-prev-icon","carousel-control-next-icon",
    "carousel-indicators","carousel-caption",
    // Dropdown
    "dropdown","dropdown-toggle","dropdown-toggle-split","dropdown-menu","dropdown-item",
    "dropdown-header","dropdown-divider","dropdown-item-text","dropdown-menu-dark","dropdown-center",
    "dropdown-menu-start","dropdown-menu-end",
    "dropdown-menu-sm-start","dropdown-menu-sm-end","dropdown-menu-md-start","dropdown-menu-md-end",
    "dropdown-menu-lg-start","dropdown-menu-lg-end","dropdown-menu-xl-start","dropdown-menu-xl-end",
    "dropdown-menu-xxl-start","dropdown-menu-xxl-end",
    "dropup","dropup-center","dropend","dropstart",
    // Forms
    "form-label","form-control","form-control-lg","form-control-sm","form-control-plaintext",
    "form-control-color","form-text","form-select","form-select-lg","form-select-sm",
    "form-check","form-check-input","form-check-label","form-check-inline","form-check-reverse",
    "form-switch","form-range","form-floating",
    "input-group","input-group-lg","input-group-sm","input-group-text","has-validation",
    "invalid-feedback","valid-feedback","invalid-tooltip","valid-tooltip",
    // Modal
    "modal","modal-dialog","modal-content","modal-header","modal-title","modal-body","modal-footer",
    "modal-sm","modal-lg","modal-xl","modal-fullscreen",
    "modal-fullscreen-sm-down","modal-fullscreen-md-down","modal-fullscreen-lg-down",
    "modal-fullscreen-xl-down","modal-fullscreen-xxl-down",
    "modal-dialog-scrollable","modal-dialog-centered","modal-backdrop","modal-open","modal-static",
    // Navbar
    "navbar","navbar-brand","navbar-toggler","navbar-toggler-icon","navbar-collapse",
    "navbar-nav","navbar-nav-scroll","navbar-text","navbar-dark","navbar-light",
    "navbar-expand","navbar-expand-sm","navbar-expand-md","navbar-expand-lg","navbar-expand-xl","navbar-expand-xxl",
    // Nav / Tabs
    "nav","nav-link","nav-tabs","nav-pills","nav-fill","nav-justified","nav-underline",
    "tab-content","tab-pane",
    // Offcanvas
    "offcanvas","offcanvas-body","offcanvas-header","offcanvas-title",
    "offcanvas-start","offcanvas-end","offcanvas-top","offcanvas-bottom",
    "offcanvas-sm","offcanvas-md","offcanvas-lg","offcanvas-xl","offcanvas-xxl",
    // Pagination
    "pagination","pagination-lg","pagination-sm","page-item","page-link",
    // Popover / Tooltip
    "popover","popover-header","popover-body",
    "bs-popover-top","bs-popover-end","bs-popover-bottom","bs-popover-start","bs-popover-auto",
    "tooltip","tooltip-inner",
    "bs-tooltip-top","bs-tooltip-end","bs-tooltip-bottom","bs-tooltip-start","bs-tooltip-auto",
    // Progress
    "progress","progress-bar","progress-bar-striped","progress-bar-animated","progress-stacked",
    // Spinner
    "spinner-border","spinner-border-sm","spinner-grow","spinner-grow-sm",
    // Table
    "table","table-sm","table-bordered","table-borderless","table-striped","table-striped-columns",
    "table-hover","table-active","table-dark","table-group-divider",
    "table-primary","table-secondary","table-success","table-danger","table-warning","table-info","table-light",
    "table-responsive","table-responsive-sm","table-responsive-md","table-responsive-lg","table-responsive-xl","table-responsive-xxl",
    // Toast
    "toast","toast-header","toast-body","toast-container",
];
