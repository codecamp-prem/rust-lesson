#[derive(Copy, Clone)]
struct LuggageId(usize);

struct Luggage<State> {
    id: LuggageId,
    state: State,
}

impl<State> Luggage<State> {
    fn next<Next>(self, state: Next) -> Luggage<Next> {
        Luggage { id: self.id, state }
    }
}

struct BeginCustody;
struct CheckIn;
struct OnLoad;
struct OffLoad;
struct AwaitingPickup;
struct EndCustody(LuggageId);

impl Luggage<BeginCustody> {
    fn new(id: LuggageId) -> Self {
        Self {
            id,
            state: BeginCustody,
        }
    }
    fn check_in(self) -> Luggage<CheckIn> {
        self.next(CheckIn)
    }
}

impl Luggage<CheckIn> {
    fn onload(self) -> Luggage<OnLoad> {
        self.next(OnLoad)
    }
}

impl Luggage<OnLoad> {
    fn offload(self) -> Luggage<OffLoad> {
        self.next(OffLoad)
    }
}

impl Luggage<OffLoad> {
    fn carousel(self) -> Luggage<AwaitingPickup> {
        self.next(AwaitingPickup)
    }
}

impl Luggage<AwaitingPickup> {
    fn pickup(self) -> (Luggage<EndCustody>, EndCustody) {
        let id = self.id;
        (self.next(EndCustody(id)), EndCustody(id))
    }
}

fn main() {
    let id = LuggageId(1);
    let luggage = Luggage::new(id);
    let luggage = luggage.check_in().onload().offload().carousel();
    luggage.pickup();
}