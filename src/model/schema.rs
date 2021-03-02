table! {
    fs (scrapbook_id, id) {
        scrapbook_id -> Integer,
        id -> Integer,
        created -> Timestamp,
        modified -> Timestamp,
        rdf_id -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        source -> Nullable<Text>,
        icon -> Nullable<Text>,
        comment -> Nullable<Text>,
        encoding -> Nullable<Text>,
        marked -> Bool,
        locked -> Bool,
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

allow_tables_to_appear_in_same_query!(
    fs,
    scrapbooks,
);
