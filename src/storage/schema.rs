table! {
    fs (id) {
        id -> Integer,
        is_root -> Bool,
        scrapbook_id -> Integer,
        rdf_id -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        title -> Nullable<Text>,
        source -> Nullable<Text>,
        icon -> Nullable<Text>,
        comment -> Nullable<Text>,
        encoding -> Nullable<Text>,
        marked -> Nullable<Bool>,
        locked -> Nullable<Bool>,
        created -> Nullable<Timestamp>,
        modified -> Nullable<Timestamp>,
        children -> Nullable<Binary>,
    }
}

table! {
    scrapbooks (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(fs -> scrapbooks (scrapbook_id));

allow_tables_to_appear_in_same_query!(fs, scrapbooks,);
