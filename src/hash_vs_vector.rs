use std::time::Instant;
use std::collections::HashMap;

pub fn hash_vs_vector_on_strings(){
    let vector_declaration_now = Instant::now();
    let kamus_vector = vec!["aba", "abad", "abadi", "abadiah", "abah", "abai", "abaimana", "abaka",
    "abaktinal", "abakus", "abal-abal", "aban", "abang", "abangan", "abangga", "abar",
    "abatoar", "abau", "abdas", "abdi", "abdikasi", "abdomen", "abdominal", "abdu",
    "abduksi", "abduktor", "abece", "aben", "aberasi", "abet", "abian", "abid",
    "abidin", "abilah", "abing", "abiogenesis", "abiosfer", "abiotik", "abis", "abisal",
    "abiseka", "abiturien", "abjad", "abjadiah", "ablasi", "ablaut", "ablepsia", "abnormal",
    "abnormalitas", "abnus", "aboi", "abolisi", "abon", "abonemen", "abong-abong", "aborsi",
    "abortif", "abortiva", "abortus", "abrak", "abrakadabra", "abrar", "abras", "abrasi",
    "abreaksi", "abrek", "abreviasi", "abrikos", "abrit-abrit", "abrosfer", "absah", "absen",
    "absensi", "absensia", "absente", "absenteisme", "abses", "absis", "absolusi", "absolut",
    "absolutisme", "absonan", "absorb", "absorben", "absorbir", "absorpsi", "absorpsiometer", "absorptif",
    "abstain", "abstinensi", "abstrak", "abstraksi", "absurd", "absurdisme", "abtar", "abu",
    "abuan", "abuh", "abuk", "abulhayat", "abulia", "abun-abun", "abur", "abus",
    "abyad", "acah", "acak", "acala", "acan", "acang", "acap", "acar",
    "acara", "acaram", "acat", "acau", "acawi", "acerang", "aci", "acik",
    "aco", "acu", "acuh", "acum", "acung", "ada", "adab", "adad"];
    let vector_declaration_elapsed = vector_declaration_now.elapsed();
    println!("Elapsed time: {:.2?}", vector_declaration_elapsed);

    let hash_declaration_now = Instant::now();
    let kamus_hash : HashMap<&str, i8> =
    [("aba", 1), ("abad", 1), ("abadi", 1), ("abadiah", 1), ("abah", 1), ("abai", 1), ("abaimana", 1), ("abaka", 1),
    ("abaktinal", 1), ("abakus", 1), ("abal-abal", 1), ("aban", 1), ("abang", 1), ("abangan", 1), ("abangga", 1), ("abar", 1),
    ("abatoar", 1), ("abau", 1), ("abdas", 1), ("abdi", 1), ("abdikasi", 1), ("abdomen", 1), ("abdominal", 1), ("abdu", 1),
    ("abduksi", 1), ("abduktor", 1), ("abece", 1), ("aben", 1), ("aberasi", 1), ("abet", 1), ("abian", 1), ("abid", 1),
    ("abidin", 1), ("abilah", 1), ("abing", 1), ("abiogenesis", 1), ("abiosfer", 1), ("abiotik", 1), ("abis", 1), ("abisal", 1),
    ("abiseka", 1), ("abiturien", 1), ("abjad", 1), ("abjadiah", 1), ("ablasi", 1), ("ablaut", 1), ("ablepsia", 1), ("abnormal", 1),
    ("abnormalitas", 1), ("abnus", 1), ("aboi", 1), ("abolisi", 1), ("abon", 1), ("abonemen", 1), ("abong-abong", 1), ("aborsi", 1),
    ("abortif", 1), ("abortiva", 1), ("abortus", 1), ("abrak", 1), ("abrakadabra", 1), ("abrar", 1), ("abras", 1), ("abrasi", 1),
    ("abreaksi", 1), ("abrek", 1), ("abreviasi", 1), ("abrikos", 1), ("abrit-abrit", 1), ("abrosfer", 1), ("absah", 1), ("absen", 1),
    ("absensi", 1), ("absensia", 1), ("absente", 1), ("absenteisme", 1), ("abses", 1), ("absis", 1), ("absolusi", 1), ("absolut", 1),
    ("absolutisme", 1), ("absonan", 1), ("absorb", 1), ("absorben", 1), ("absorbir", 1), ("absorpsi", 1), ("absorpsiometer", 1), ("absorptif", 1),
    ("abstain", 1), ("abstinensi", 1), ("abstrak", 1), ("abstraksi", 1), ("absurd", 1), ("absurdisme", 1), ("abtar", 1), ("abu", 1),
    ("abuan", 1), ("abuh", 1), ("abuk", 1), ("abulhayat", 1), ("abulia", 1), ("abun-abun", 1), ("abur", 1), ("abus", 1),
    ("abyad", 1), ("acah", 1), ("acak", 1), ("acala", 1), ("acan", 1), ("acang", 1), ("acap", 1), ("acar", 1),
    ("acara", 1), ("acaram", 1), ("acat", 1), ("acau", 1), ("acawi", 1), ("acerang", 1), ("aci", 1), ("acik", 1),
    ("aco", 1), ("acu", 1), ("acuh", 1), ("acum", 1), ("acung", 1), ("ada", 1), ("adab", 1), ("adad", 1)].iter().cloned().collect();
    let hash_declaration_elapsed = hash_declaration_now.elapsed();
    println!("Elapsed time: {:.2?}", hash_declaration_elapsed);

    if vector_declaration_elapsed > hash_declaration_elapsed {
        println!("hash WIN!, vector declaration takes longer time");
    } else {
        println!("vector WIN!, hash declaration takes longer time {}% difference, vector {} nanosecond and hash {} nanosecond or {} microsecond", 
        ((hash_declaration_elapsed.as_nanos() - vector_declaration_elapsed.as_nanos()) / vector_declaration_elapsed.as_nanos()) * 100, vector_declaration_elapsed.as_nanos(), vector_declaration_elapsed.as_nanos(), vector_declaration_elapsed.as_micros());
    }


    println!("=========================================");
    let vector_now = Instant::now();
    kamus_vector.iter().position(|&x| x == "acan");
    let vector_elapsed = vector_now.elapsed();
    println!("Elapsed time: {:.2?}", vector_elapsed);

    let hash_find_now = Instant::now();
    kamus_hash["acan"];
    let hash_find_elapsed = hash_find_now.elapsed();
    println!("Elapsed time: {:.2?}", hash_find_elapsed);

    if vector_elapsed > hash_find_elapsed {
        println!("hash WIN!, vector finding takes longer time {}% difference, vector {} nanosecond and hash {} nanosecond", 
        ((vector_elapsed.as_nanos() - hash_find_elapsed.as_nanos()) / hash_find_elapsed.as_nanos()) * 100, vector_elapsed.as_nanos(), hash_find_elapsed.as_nanos());
    } else {
        println!("vector WIN!, hash finding takes longer time");
    }
}