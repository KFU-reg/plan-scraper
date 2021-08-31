use serde_json::json;

pub const MAJORS_URLS: &[(&str, &str)] = &[
    (
	"Elect",
	"https://www.kfu.edu.sa/ar/Deans/AdmissionRecordsDeanship/Documents/acadPlan/progs/p_22_%D8%A8%D9%83_%D9%83%D9%87%D8%B1%D8%A8%D8%A7%D8%A1_11.html"
    ),

    (
	"Chema",
	"https://www.kfu.edu.sa/ar/Deans/AdmissionRecordsDeanship/Documents/acadPlan/progs/p_22_%D8%A8%D9%83_%D9%83%D9%8A%D9%85%D9%8A%D8%A7%D8%A6_11.html"
    ),

    (
	"Mecha",
	"https://www.kfu.edu.sa/ar/Deans/AdmissionRecordsDeanship/Documents/acadPlan/progs/p_22_%D8%A8%D9%83_%D9%85%D9%8A%D9%83%D8%A7%D9%86%D9%8A_11.html"
    ),

    (
	"Civil",
	"https://www.kfu.edu.sa/ar/Deans/AdmissionRecordsDeanship/Documents/acadPlan/progs/p_22_%D8%A8%D9%83_%D9%85%D8%AF%D9%86%D9%8A%D8%A9_11.html"
    ),

];

// from https://www.kfu.edu.sa/ar/Deans/AdmissionRecordsDeanship/Documents/acadPlan/studyTables.html?cry=1442&sm=10#
// fn avialable_courses_urls() -> serde_json::Value {
//     json!([
//         { "name": "الطب", "p_col_code": "10", "sex": "q", },
//         { "name": "العلوم الطبية التطبيقية", "p_col_code": "31", "sex": "b", },
//         { "name": "الأسنان", "p_col_code": "34", "sex": "b", },
//         { "name": "الصيدلة الإكلينيكية", "p_col_code": "20", "sex": "b", },
//         { "name": "الطب البيطري", "p_col_code": "03", "sex": "b", }, //, stat: "-1"
//         { "name": "الهندسة", "p_col_code": "22", "sex": "b", },
//         { "name": "علوم الحاسب و تقنية المعلومات", "p_col_code": "09", "sex": "b", }, //, stat: "-1"
//         { "name": "إدارة الأعمال", "p_col_code": "06", "sex": "b", },
//         { "name": "العلوم", "p_col_code": "08", "sex": "b", },
//         { "name": "العلوم الزراعية", "p_col_code": "01", "sex": "b", },
//         { "name": "التربية", "p_col_code": "02", "sex": "b", },
//         { "name": "الحقوق", "p_col_code": "27", "sex": "b", },
//         { "name": "الآداب", "p_col_code": "74", "sex": "b", },
//         { "name": "الدراسات التطبيقية وخدمة المجتمع", "p_col_code": "30", "sex": "b", },
//         { "name": "المجتمع ببقيق", "p_col_code": "28", "sex": "b", },
//         { "name": "مركز اللغة الإنجليزية", "p_col_code": "00", "sex": "b", }
//         //{ name: "الأقسام العلمية", "p_col_code": "71", "sex": "f", },
//         //{ name: "مركز الدراسات<br>المساندة", "p_col_code": "19", "sex": "b", }
//         //{ name: "الأقسام الأدبية", "p_col_code": "70", "sex": "f", }

//     ])
//     // https://banner.kfu.edu.sa:7710/KFU/ws?p_trm_code=144210&p_col_code=20&p_sex_code=11
// }
