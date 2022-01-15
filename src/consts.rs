use serde_json::json;
// sm=10 is semster 1
// sm=10 is semster 2
// sex_code=11 is males, 12 is females

pub const MAJORS_URLS: &[(&str, &str)] = &[

    (
	"EE",
	"https://www.kfu.edu.sa/ar/Deans/AdmissionRecordsDeanship/Documents/acadPlan/progs/p_22_%D8%A8%D9%83_%D9%83%D9%87%D8%B1%D8%A8%D8%A7%D8%A1_11.html"
    ),

    (
	"ChE",
	"https://www.kfu.edu.sa/ar/Deans/AdmissionRecordsDeanship/Documents/acadPlan/progs/p_22_%D8%A8%D9%83_%D9%83%D9%8A%D9%85%D9%8A%D8%A7%D8%A6_11.html"
    ),

    (
	"ME",
	"https://www.kfu.edu.sa/ar/Deans/AdmissionRecordsDeanship/Documents/acadPlan/progs/p_22_%D8%A8%D9%83_%D9%85%D9%8A%D9%83%D8%A7%D9%86%D9%8A_11.html"
    ),

    (
	"CE",
	"https://www.kfu.edu.sa/ar/Deans/AdmissionRecordsDeanship/Documents/acadPlan/progs/p_22_%D8%A8%D9%83_%D9%85%D8%AF%D9%86%D9%8A%D8%A9_11.html"
    ),

];
pub const CLASS_LIST_PROPS: &[ClassListURLS] = &[
    // ClassListURLS {name: "الطب", code: "10", is_male: "q",},
    // ClassListURLS {
    //     name_ar: "العلوم الطبية التطبيقية",
    //     code: "31",
    //     is_male: true,
    // },
    // ClassListURLS {
    //     name_ar: "الأسنان",
    //     code: "34",
    //     is_male: true,
    // },
    // ClassListURLS {
    //     name_ar: "الصيدلة الإكلينيكية",
    //     code: "20",
    //     is_male: true,
    // },
    // ClassListURLS {
    //     name_ar: "الطب البيطري",
    //     code: "03",
    //     is_male: true,
    // }, //, stat: "-1"
    ClassListURLS {
        name_ar: "الهندسة",
        code: "22",
        is_male: true,
        name_en: "Engineering",
    },
    ClassListURLS {
        name_ar: "علوم الحاسب و تقنية المعلومات",
        code: "09",
        is_male: true,
        name_en: "Computer Science",
    }, //, stat: "-1"
    ClassListURLS {
        name_ar: "إدارة الأعمال",
        code: "06",
        is_male: true,
        name_en: "Business",
    },
    ClassListURLS {
        name_ar: "العلوم",
        code: "08",
        is_male: true,
        name_en: "Science",
    },
    ClassListURLS {
        name_ar: "العلوم الزراعية",
        code: "01",
        is_male: true,
        name_en: "AgriculturalSciences",
    },
    ClassListURLS {
        name_ar: "التربية",
        code: "02",
        is_male: true,
        name_en: "Education",
    },
    ClassListURLS {
        name_ar: "الحقوق",
        code: "27",
        is_male: true,
        name_en: "Law",
    },
    ClassListURLS {
        name_ar: "الآداب",
        code: "74",
        is_male: true,
        name_en: "Arts",
    },
    ClassListURLS {
        name_ar: "الدراسات التطبيقية وخدمة المجتمع",
        code: "30",
        is_male: true,
        name_en: "AppliedSciences",
    },
    // ClassListURLS {
    //     name_ar: "المجتمع ببقيق",
    //     code: "28",
    //     is_male: true,
    // },
    ClassListURLS {
        name_ar: "مركز اللغة الإنجليزية",
        code: "00",
        is_male: true,
        name_en: "English",
    },
    // ClassListURLS {
    //     name_ar: "الأقسام العلمية",
    //     code: "71",
    //     is_male: false,
    // },
    // ClassListURLS {
    //     name_ar: "مركز الدراسات<br>المساندة",
    //     code: "19",
    //     is_male: true,
    // },
    // ClassListURLS {
    //     name_ar: "الأقسام الأدبية",
    //     code: "70",
    //     is_male: false,
    // },
];

// from https://www.kfu.edu.sa/ar/Deans/AdmissionRecordsDeanship/Documents/acadPlan/studyTables.html?cry=1442&sm=10#
pub fn avialable_courses_urls() -> Vec<String> {
    // const BASE_URL: &str =
    //     "https://banner.kfu.edu.sa:7710/KFU/ws?p_trm_code=144210&p_col_code=20&p_sex_code=11";
    const BASE_URL: &str = "https://banner.kfu.edu.sa:7710/KFU/ws?";
    let mut urls = vec![];

    for prop in CLASS_LIST_PROPS {
        let sex_code = match prop.is_male {
            true => 11,
            false => 12,
        };
        let url = format!(
            "{}p_trm_code={}{}&p_col_code={}&p_sex_code={}",
            BASE_URL, 1442, 20, prop.code, sex_code
        );
        urls.push(url);
    }

    println!("{:#?}", urls);
    urls
}

#[derive(Debug, Clone, Copy)]
pub struct ClassListURLS<'a> {
    pub code: &'a str,
    pub is_male: bool,
    pub name_en: &'a str,
    pub name_ar: &'a str,
}
