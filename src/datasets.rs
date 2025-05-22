// This is free and unencumbered software released into the public domain.

use crate::dataset::Dataset;

pub static URL_PREFIX_TO_DATASET: [(&str, Dataset); 26] = [
    (
        "https://finance.yahoo.com/quote/",
        Dataset {
            id: "gd_lmrpz3vxmz972ghd7",
            label: "Yahoo Finance",
        },
    ),
    (
        "https://www.airbnb.com/rooms/",
        Dataset {
            id: "gd_ld7ll037kqy322v05",
            label: "Airbnb",
        },
    ),
    (
        "https://www.amazon.com/",
        Dataset {
            id: "gd_l7q7dkf244hwjntr0",
            label: "Amazon",
        },
    ),
    (
        "https://www.amazon.com/sp?seller=",
        Dataset {
            id: "gd_lhotzucw1etoe5iw1k",
            label: "Amazon",
        },
    ),
    (
        "https://www.crunchbase.com/organization/",
        Dataset {
            id: "gd_l1vijqt9jfj7olije",
            label: "Crunchbase",
        },
    ),
    (
        "https://www.ebay.com/itm/",
        Dataset {
            id: "gd_ltr9mjt81n0zzdk1fb",
            label: "eBay",
        },
    ),
    (
        "https://www.facebook.com/events/",
        Dataset {
            id: "gd_m14sd0to1jz48ppm51",
            label: "Facebook",
        },
    ),
    (
        "https://www.facebook.com/groups/",
        Dataset {
            id: "gd_lz11l67o2cb3r0lkj3",
            label: "Facebook",
        },
    ),
    (
        "https://www.facebook.com/marketplace/item/",
        Dataset {
            id: "gd_lvt9iwuh6fbcwmx1a",
            label: "Facebook",
        },
    ),
    (
        "https://www.facebook.com/share/p/",
        Dataset {
            id: "gd_lyclm1571iy3mv57zw",
            label: "Facebook",
        },
    ),
    (
        "https://www.google.com/shopping/product/",
        Dataset {
            id: "gd_ltppk50q18kdw67omz",
            label: "Google Shopping",
        },
    ),
    (
        "https://www.indeed.com/cmp/",
        Dataset {
            id: "gd_l7qekxkv2i7ve6hx1s",
            label: "Indeed",
        },
    ),
    (
        "https://www.instagram.com/",
        Dataset {
            id: "gd_l1vikfch901nx3by4",
            label: "Instagram",
        },
    ),
    (
        "https://www.instagram.com/p/",
        Dataset {
            id: "gd_lk5ns7kz21pck8jpis",
            label: "Instagram",
        },
    ),
    (
        "https://www.instagram.com/reel/",
        Dataset {
            id: "gd_lyclm20il4r5helnj",
            label: "Instagram",
        },
    ),
    (
        "https://www.linkedin.com/company/",
        Dataset {
            id: "gd_l1vikfnt1wgvvqz95w",
            label: "LinkedIn",
        },
    ),
    //("https://www.linkedin.com/feed/update/", "gd_lyy3tktm25m4avu764"), // TODO
    (
        "https://www.linkedin.com/in/",
        Dataset {
            id: "gd_l1viktl72bvl7bjuj0",
            label: "LinkedIn",
        },
    ),
    (
        "https://www.linkedin.com/jobs/",
        Dataset {
            id: "gd_lpfll7v5hcqtkxl6l",
            label: "LinkedIn",
        },
    ),
    (
        "https://www.linkedin.com/posts/",
        Dataset {
            id: "gd_lyy3tktm25m4avu764",
            label: "LinkedIn",
        },
    ),
    (
        "https://www.linkedin.com/pulse/",
        Dataset {
            id: "gd_lyy3tktm25m4avu764",
            label: "LinkedIn",
        },
    ),
    (
        "https://www.walmart.com/global/seller/",
        Dataset {
            id: "gd_m7ke48w81ocyu4hhz0",
            label: "Walmart",
        },
    ),
    (
        "https://www.walmart.com/ip/",
        Dataset {
            id: "gd_l95fol7l1ru6rlo116",
            label: "Walmart",
        },
    ),
    (
        "https://www.youtube.com/@",
        Dataset {
            id: "gd_lk538t2k2p1k3oos71",
            label: "YouTube",
        },
    ),
    (
        "https://www.youtube.com/watch?v=",
        Dataset {
            id: "gd_lk56epmy2i5g7lzu0k",
            label: "YouTube",
        },
    ),
    (
        "https://x.com/",
        Dataset {
            id: "gd_lwxmeb2u1cniijd7t4",
            label: "X",
        },
    ),
    (
        "https://x.com/bendiken/status/", // TODO
        Dataset {
            id: "gd_lwxkxvnf1cynvib9co",
            label: "X",
        },
    ),
];
