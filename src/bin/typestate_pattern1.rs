// Topic: Typestates

// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)

#[derive(Copy, Clone)]
struct LuggageId(usize);

struct Luggage(LuggageId);
impl Luggage {
    fn new(id: LuggageId) -> Self{
        Luggage(id)
    }

    fn check_in(self) -> CheckIn{
        CheckIn(self.0)
    }
}

struct CheckIn(LuggageId);
impl CheckIn{
    fn onload(self) -> OnLoading{
        OnLoading(self.0)
    }
}

struct OnLoading(LuggageId);
impl OnLoading{
    fn offload(self) -> OffLoading{
        OffLoading(self.0)
    }
}

struct OffLoading(LuggageId);
impl OffLoading{
    fn carousel(self) -> AwaitingPickup{
        AwaitingPickup(self.0)
    }
}

struct AwaitingPickup(LuggageId);
impl AwaitingPickup{
    fn pickup(self) -> (Luggage, EndCustody){
        (Luggage(self.0), EndCustody(self.0))
    }
}

struct EndCustody(LuggageId);

fn main() {
    let id = LuggageId(1);
    let luggage = Luggage::new(id);
    let luggage = luggage.check_in().onload().offload().carousel();
    let (luggage, _) = luggage.pickup();
}