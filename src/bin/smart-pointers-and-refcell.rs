// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Vehicle{
    Car,
    Truck,
}

#[derive(Debug, Hash, PartialOrd, PartialEq)]
enum Status{
    Available,
    Unavailable,
    Rented,
    Maintenance,
}

#[derive(Debug)]
struct Rentals{
    status: Status,
    vehicle: Vehicle,
    vin: String,
}

struct Corporate(Rc<RefCell<Vec<Rentals>>>);

struct StoreFront(Rc<RefCell<Vec<Rentals>>>);

fn main() {}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn update_status(){
        let vehicles = vec![
            Rentals{
                status: Status::Available,
                vehicle: Vehicle::Truck,
                vin: "123".to_owned(),
            },
            Rentals{
                status: Status::Maintenance,
                vehicle: Vehicle::Car,
                vin: "456".to_owned(),
            }, 
        ];

        let vehicles = Rc::new(RefCell::new(vehicles));

        let corporate = Corporate(Rc::clone(&vehicles));
        let store_front = StoreFront(Rc::clone(&vehicles));

        {
            let mut rentals = store_front.0.borrow_mut();
            if let Some(truck) = rentals.get_mut(0){
                assert_eq!(truck.status, Status::Available);
                truck.status = Status::Rented;
            }
        }

        {
            let mut rentals = corporate.0.borrow_mut();
            if let Some(truck) = rentals.get_mut(0){
                assert_eq!(truck.status, Status::Rented);
                truck.status = Status::Available;
            }
        }

        let rentals = store_front.0.borrow();
        if let Some(truck) = rentals.get(0){
            assert_eq!(truck.status, Status::Available);
        }
    }
}