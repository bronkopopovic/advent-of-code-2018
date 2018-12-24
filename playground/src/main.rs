fn main() {
    
    struct Date {
        year: i32,
        month: i32,
        day: i32,
    }

    let mut dates: Vec<Date> = Vec::new();

    dates.push( Date {year: 2018, month: 10, day: 2} );
    dates.push( Date {year: 2018, month: 10, day: 12} );
    dates.push( Date {year: 2018, month: 10, day: 1} );
    dates.push( Date {year: 2018, month: 10, day: 24} );
    dates.push( Date {year: 2017, month: 9, day: 13} );
    dates.push( Date {year: 2019, month: 2, day: 25} );
    dates.push( Date {year: 2018, month: 12, day: 2} );
    dates.push( Date {year: 2018, month: 5, day: 10} );
    dates.push( Date {year: 2017, month: 8, day: 5} );
    dates.push( Date {year: 2017, month: 8, day: 2} );
    dates.push( Date {year: 2017, month: 8, day: 18} );
    dates.push( Date {year: 2017, month: 8, day: 9} );
    dates.push( Date {year: 2019, month: 11, day: 2} );
    dates.push( Date {year: 2018, month: 2, day: 4} );

    for _date in &dates {
        println!("{}-{}", _date.year, _date.month);
    }

    println!("Now the sorted stuff:");

    dates.sort_by( |l, r| {
        l.year.cmp(&r.year)
            .then(l.month.cmp(&r.month))
                .then(l.day.cmp(&r.day))
    });

    for _date in &dates {
        println!("{}-{}-{}", _date.year, _date.month, _date.day);
    }

    // let mut nums = vec![2, 3, 44, 13, 56, 24, 64, 75, 32];

    // nums.sort_by(|l, r| if l == r {
    //     r.cmp(&l)
    // } else {
    //     l.cmp(&r)
    // });
    
    // println!("{:?}", nums);
}