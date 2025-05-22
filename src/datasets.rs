// This is free and unencumbered software released into the public domain.

pub static URL_PREFIX_TO_DATASET_ID: [(&str, &str); 26] = [
    ("https://finance.yahoo.com/quote/", "gd_lmrpz3vxmz972ghd7"),
    ("https://www.airbnb.com/rooms/", "gd_ld7ll037kqy322v05"),
    ("https://www.amazon.com/", "gd_l7q7dkf244hwjntr0"),
    ("https://www.amazon.com/sp?seller=", "gd_lhotzucw1etoe5iw1k"),
    (
        "https://www.crunchbase.com/organization/",
        "gd_l1vijqt9jfj7olije",
    ),
    ("https://www.ebay.com/itm/", "gd_ltr9mjt81n0zzdk1fb"),
    ("https://www.facebook.com/events/", "gd_m14sd0to1jz48ppm51"),
    ("https://www.facebook.com/groups/", "gd_lz11l67o2cb3r0lkj3"),
    (
        "https://www.facebook.com/marketplace/item/",
        "gd_lvt9iwuh6fbcwmx1a",
    ),
    ("https://www.facebook.com/share/p/", "gd_lyclm1571iy3mv57zw"),
    (
        "https://www.google.com/shopping/product/",
        "gd_ltppk50q18kdw67omz",
    ),
    ("https://www.indeed.com/cmp/", "gd_l7qekxkv2i7ve6hx1s"),
    ("https://www.instagram.com/", "gd_l1vikfch901nx3by4"),
    ("https://www.instagram.com/p/", "gd_lk5ns7kz21pck8jpis"),
    ("https://www.instagram.com/reel/", "gd_lyclm20il4r5helnj"),
    ("https://www.linkedin.com/company/", "gd_l1vikfnt1wgvvqz95w"),
    //("https://www.linkedin.com/feed/update/", "gd_lyy3tktm25m4avu764"), // TODO
    ("https://www.linkedin.com/in/", "gd_l1viktl72bvl7bjuj0"),
    ("https://www.linkedin.com/jobs/", "gd_lpfll7v5hcqtkxl6l"),
    ("https://www.linkedin.com/posts/", "gd_lyy3tktm25m4avu764"),
    ("https://www.linkedin.com/pulse/", "gd_lyy3tktm25m4avu764"),
    (
        "https://www.walmart.com/global/seller/",
        "gd_m7ke48w81ocyu4hhz0",
    ),
    ("https://www.walmart.com/ip/", "gd_l95fol7l1ru6rlo116"),
    ("https://www.youtube.com/@", "gd_lk538t2k2p1k3oos71"),
    ("https://www.youtube.com/watch?v=", "gd_lk56epmy2i5g7lzu0k"),
    ("https://x.com/", "gd_lwxmeb2u1cniijd7t4"),
    ("https://x.com/bendiken/status/", "gd_lwxkxvnf1cynvib9co"), // TODO
];
