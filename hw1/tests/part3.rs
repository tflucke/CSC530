#[cfg(test)]
mod part3_tests {
   use std::collections::HashMap;
   use hw1::data;
   use data::Ticket;
   use hw1::part3::tally_tickets;

   #[test]
   fn check_tally_ticket1() {
      let tickets = vec![
            Ticket {event_id: String::from("xyazn123"),
                     section_id: String::from("A101"),
                     row_id: String::from("C"),
                     seat_id: 17,},
            Ticket {event_id: String::from("dcc21x"),
                     section_id: String::from("C102"),
                     row_id: String::from("AA"),
                     seat_id: 12,},
            Ticket {event_id: String::from("cpfr27"),
                     section_id: String::from("A101"),
                     row_id: String::from("C"),
                     seat_id: 17,},
            Ticket {event_id: String::from("xyazn123"),
                     section_id: String::from("B201"),
                     row_id: String::from("F"),
                     seat_id: 4,},
            ];

      let expected : HashMap<_, _> = 
         vec![("dcc21x", 1), ("xyazn123", 2), ("cpfr27", 1)].into_iter().collect();

      assert_eq!(tally_tickets(&tickets), expected);
   }
}

