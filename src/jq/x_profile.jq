.[] | {
  "@context": {
    "know": "https://know.dev/",
    "xsd": "http://www.w3.org/2001/XMLSchema#",
    "count": {
      "@id": "know:count",
      "@type": "xsd:integer",
    },
    "followees": {
      "@id": "know:followees",
      "@type": "know:Collection",
    },
    "followers": {
      "@id": "know:followers",
      "@type": "know:Collection",
    },
    "id": {
      "@id": "know:id",
      "@type": "xsd:string",
    },
    "image": {
      "@id": "know:image",
      "@type": "@id",
    },
    "link": {
      "@id": "know:link",
      "@type": "@id",
    },
    "name": {
      "@id": "know:name",
      "@type": "xsd:string",
    },
    "posts": {
      "@id": "know:posts",
      "@type": "know:Collection",
    },
    "subscriptions": {
      "@id": "know:subscriptions",
      "@type": "know:Collection",
    },
    "summary": {
      "@id": "know:summary",
      "@language": "en",
    },
    "title": {
      "@id": "know:title",
      "@language": "en",
    },
  },
  "@id": .url,
  "@type": ["know:UserProfile", "know:UserAccount"],
  "id": .x_id,
  "name": .url | sub("https://x\\.com/"; ""),
  "title": .profile_name,
  "summary": .biography,
  "image": .profile_image_link,
  "link": .external_link,
  "followees": {
    "@type": "know:Collection",
    "count": .following,
  },
  "followers": {
    "@type": "know:Collection",
    "count": .followers
  },
  "posts": {
    "@type": "know:Collection",
    "count": .posts_count,
  },
  "subscriptions": {
    "@type": "know:Collection",
    "count": .subscriptions,
  },
}
