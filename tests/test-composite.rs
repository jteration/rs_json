#[cfg(test)]
mod tests {
  use rs_json::run;
  use rs_json::JsonValue;

  #[test]
  fn test_good_composite_1() {
    let path = "./tests/good_json/composite/test-1.json".to_string();
    let good_composite_one: JsonValue = run(&path).unwrap();

    match good_composite_one {
      JsonValue::JObj(good_composite_one_val) => {
        let image_obj = good_composite_one_val.get("Image").unwrap();

        match image_obj {
          JsonValue::JObj(image_obj_val) => {
            let width = image_obj_val.get("Width").unwrap();
            let height = image_obj_val.get("Height").unwrap();
            let title = image_obj_val.get("Title").unwrap();
            let thumbnail_obj = image_obj_val.get("Thumbnail").unwrap();
            let animated = image_obj_val.get("Animated").unwrap();
            let ids_array = image_obj_val.get("IDs").unwrap();

            match width {
              JsonValue::JNum(width_val) => assert_eq!(*width_val, 800.0f64),
              _ => assert!(false),
            };

            match height {
              JsonValue::JNum(height_val) => assert_eq!(*height_val, 600.0f64),
              _ => assert!(false),
            };

            match title {
              JsonValue::JString(title_val) => assert_eq!(*title_val, "View from 15th Floor".to_string()),
              _ => assert!(false),
            };

            match thumbnail_obj {
              JsonValue::JObj(thumbnail_obj_val) => {
                let url = thumbnail_obj_val.get("Url").unwrap();
                let thumbnail_height = thumbnail_obj_val.get("Height").unwrap();
                let thumbnail_width = thumbnail_obj_val.get("Width").unwrap();

                match url {
                  JsonValue::JString(url_val) => assert_eq!(*url_val, "http://www.example.com/image/481989943".to_string()),
                  _ => assert!(false),
                };

                match thumbnail_height {
                  JsonValue::JNum(thumbnail_height_val) => assert_eq!(*thumbnail_height_val, 125.0f64),
                  _ => assert!(false),
                };

                match thumbnail_width {
                  JsonValue::JNum(thumbnail_width_val) => assert_eq!(*thumbnail_width_val, 100.0f64),
                  _ => assert!(false),
                };
              }
              _ => assert!(false),
            };

            match animated {
              JsonValue::JBool(animated_val) => assert_eq!(*animated_val, false),
              _ => assert!(false),
            };

            match ids_array {
              JsonValue::JArray(ids_array_val) => {
                assert_eq!(ids_array_val.len(), 4);
                let id_one = &ids_array_val[0];
                let id_two = &ids_array_val[1];
                let id_three = &ids_array_val[2];
                let id_four = &ids_array_val[3];

                match id_one {
                  JsonValue::JNum(id_one_val) => assert_eq!(*id_one_val, 116.0f64),
                  _ => assert!(false),
                }

                match id_two {
                  JsonValue::JNum(id_two_val) => assert_eq!(*id_two_val, 943.0f64),
                  _ => assert!(false),
                }

                match id_three {
                  JsonValue::JNum(id_three_val) => assert_eq!(*id_three_val, 234.0f64),
                  _ => assert!(false),
                }

                match id_four {
                  JsonValue::JNum(id_four_val) => assert_eq!(*id_four_val, 38793.0f64),
                  _ => assert!(false),
                }
              }
              _ => assert!(false),
            };
          }
          _ => assert!(false),
        }
      }
      _ => assert!(false),
    }
  }

  #[test]
  fn test_good_composite_2() {
    let path = "./tests/good_json/composite/test-2.json".to_string();
    let good_composite_two: JsonValue = run(&path).unwrap();

    match good_composite_two {
      JsonValue::JArray(good_composite_two_val) => {
        assert_eq!(good_composite_two_val.len(), 2);
        let obj_one = &good_composite_two_val[0];
        let obj_two = &good_composite_two_val[1];

        match obj_one {
          JsonValue::JObj(obj_one_val) => {
            let precision = obj_one_val.get("precision").unwrap();
            let latitude = obj_one_val.get("Latitude").unwrap();
            let longitude = obj_one_val.get("Longitude").unwrap();
            let address = obj_one_val.get("Address").unwrap();
            let city = obj_one_val.get("City").unwrap();
            let state = obj_one_val.get("State").unwrap();
            let zip = obj_one_val.get("Zip").unwrap();
            let country = obj_one_val.get("Country").unwrap();
            let test = obj_one_val.get("test").unwrap();

            match precision {
              JsonValue::JString(precision_val) => assert_eq!(*precision_val, "zip".to_string()),
              _ => assert!(false),
            };

            match latitude {
              JsonValue::JNum(latitude_val) => assert_eq!(*latitude_val, 37.7668f64),
              _ => assert!(false),
            };

            match longitude {
              JsonValue::JNum(longitude_val) => assert_eq!(*longitude_val, -122.3959f64),
              _ => assert!(false),
            };

            match address {
              JsonValue::JString(address_val) => assert_eq!(*address_val, "".to_string()),
              _ => assert!(false),
            };

            match city {
              JsonValue::JString(city_val) => assert_eq!(*city_val, "SAN FRANCISCO".to_string()),
              _ => assert!(false),
            };

            match state {
              JsonValue::JString(state_val) => assert_eq!(*state_val, "CA".to_string()),
              _ => assert!(false),
            };

            match zip {
              JsonValue::JString(zip_val) => assert_eq!(*zip_val, "94107".to_string()),
              _ => assert!(false),
            };

            match country {
              JsonValue::JString(country_val) => assert_eq!(*country_val, "US".to_string()),
              _ => assert!(false),
            };

            match test {
              JsonValue::JBool(test_val) => assert_eq!(*test_val, true),
              _ => assert!(false),
            };
          }
          _ => assert!(false),
        };

        match obj_two {
          JsonValue::JObj(obj_two_val) => {
            let precision = obj_two_val.get("precision").unwrap();
            let latitude = obj_two_val.get("Latitude").unwrap();
            let longitude = obj_two_val.get("Longitude").unwrap();
            let address = obj_two_val.get("Address").unwrap();
            let city = obj_two_val.get("City").unwrap();
            let state = obj_two_val.get("State").unwrap();
            let zip = obj_two_val.get("Zip").unwrap();
            let country = obj_two_val.get("Country").unwrap();
            let test = obj_two_val.get("test").unwrap();

            match precision {
              JsonValue::JString(precision_val) => assert_eq!(*precision_val, "zip".to_string()),
              _ => assert!(false),
            };

            match latitude {
              JsonValue::JNum(latitude_val) => assert_eq!(*latitude_val, 37.371991f64),
              _ => assert!(false),
            };

            match longitude {
              JsonValue::JNum(longitude_val) => assert_eq!(*longitude_val, -122.02602f64),
              _ => assert!(false),
            };

            match address {
              JsonValue::JString(address_val) => assert_eq!(*address_val, "".to_string()),
              _ => assert!(false),
            };

            match city {
              JsonValue::JString(city_val) => assert_eq!(*city_val, "SUNNYVALE".to_string()),
              _ => assert!(false),
            };

            match state {
              JsonValue::JString(state_val) => assert_eq!(*state_val, "CA".to_string()),
              _ => assert!(false),
            };

            match zip {
              JsonValue::JString(zip_val) => assert_eq!(*zip_val, "94085".to_string()),
              _ => assert!(false),
            };

            match country {
              JsonValue::JString(country_val) => assert_eq!(*country_val, "US".to_string()),
              _ => assert!(false),
            };

            match test {
              JsonValue::JBool(test_val) => assert_eq!(*test_val, false),
              _ => assert!(false),
            };
          }
          _ => assert!(false),
        };
      }
      _ => assert!(false),
    }
  }
}
