fn main() {
    let abi: f32 = 2.4;
    let abi: f32 = 120.0 - 20.0 * abi;
    
    println!("Umgewandelt HBZ: {}", abi);
    
    let gewichtung = vec![3, 2, 1, 1];
    let math = vec![13, 13, 12, 13, 13];
    let deutsch = vec![6, 7, 6, 7];
    let englisch = vec![8, 9, 8, 9, 9];
    let informatik = vec![14, 11, 12, 12, 12];
    
    let mut addit: i32 = 0;
    let mut teiler: i32 = 0;

    for g in 0..gewichtung.len() {
        let mut temp = &math;
        match g {
            0 => {temp = &math},
            1 => {temp = &deutsch},
            2 => {temp = &englisch},
            3 => {temp = &informatik},
            _ => {},
        }

        let mut add: i32 = 0;

        for x in 0..temp.len() {
            add = add + temp[x];
        }

        addit = addit + gewichtung[g] * add; 
        teiler = teiler + gewichtung[g] * temp.len() as i32;
    }
    let mut zwischenergebnis: f32 = addit as f32 / teiler as f32;
    zwischenergebnis = (zwischenergebnis * 10.0).round() / 10.0;

    let ergebnis: f32 = 10.0 + 6.0 * zwischenergebnis;
    
    println!("Addition der Einzelnoten: {}, Teiler: {}, Zwischenergebnis {}, Ergebnis {}",
             addit, teiler, zwischenergebnis, ergebnis);

    let punkte: f32 = (0.65 * abi + 0.35 * ergebnis).round();
    println!("Endpunkte (73 werden für eine direkt annahme gebraucht): {}", punkte);
}
