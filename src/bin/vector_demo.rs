struct Test{
    score: i32,
}

fn main(){
    let scores = vec! [
        Test { score: 67 },
        Test { score: 78 },
        Test { score: 76 },
        Test { score: 69 },
    ];

    for get_score in scores {
        println!("{:?}", get_score.score);
    }
}