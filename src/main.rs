use macros_derive;

// trait OrderEventSet {
//     type CreateOrderEvent;
//     type CancelOrderEvent;
//     type MatchOrderEvent;
// }

trait OrderEventVisitee<I> {
    fn accept<V>(&self, v: &V) -> Option<V::Output>
    where
        V: OrderEventVisitor<I>;
}

#[derive(Debug, Clone, macros_derive::OrderEventVisitee)]
struct LiquidCreateOrderEvent(u64);
#[derive(Debug, Clone, macros_derive::OrderEventVisitee)]
struct LiquidCancelOrderEvent(u64);
#[derive(Debug, Clone, macros_derive::OrderEventVisitee)]
struct LiquidMatchOrderEvent(u64);

#[derive(Debug, Clone, macros_derive::OrderEventVisitee)]
struct BinanceCreateOrderEvent(u64);
#[derive(Debug, Clone, macros_derive::OrderEventVisitee)]
struct BinanceCancelOrderEvent(u64);
#[derive(Debug, Clone, macros_derive::OrderEventVisitee)]
struct BinanceMatchOrderEvent(u64);

#[derive(Debug, Clone, macros_derive::OrderEventVisitee)]
struct CoinbaseCreateOrderEvent(u64);
#[derive(Debug, Clone, macros_derive::OrderEventVisitee)]
struct CoinbaseCancelOrderEvent(u64);
#[derive(Debug, Clone, macros_derive::OrderEventVisitee)]
struct CoinbaseMatchOrderEvent(u64);

#[derive(Debug)]
struct InternalCreateOrderEvent(u64);
#[derive(Debug)]
struct InternalCancelOrderEvent(u64);
#[derive(Debug)]
struct InternalMatchOrderEvent(u64);

// struct LiquidOrderEventSet;
// impl OrderEventSet for LiquidOrderEventSet {
//     type CreateOrderEvent = LiquidCreateOrderEvent;
//     type CancelOrderEvent = LiquidCancelOrderEvent;
//     type MatchOrderEvent = LiquidMatchOrderEvent;
// }
// struct BinanceOrderEventSet;
// impl OrderEventSet for BinanceOrderEventSet {
//     type CreateOrderEvent = BinanceCreateOrderEvent;
//     type CancelOrderEvent = BinanceCancelOrderEvent;
//     type MatchOrderEvent = BinanceMatchOrderEvent;
// }
//
// struct InternalOrderEventSet;
// impl OrderEventSet for InternalOrderEventSet {
//     type CreateOrderEvent = InternalCreateOrderEvent;
//     type CancelOrderEvent = InternalCancelOrderEvent;
//     type MatchOrderEvent = InternalMatchOrderEvent;
// }

#[derive(Debug)]
enum LiquidOrderEvent {
    Create(LiquidCreateOrderEvent),
    Cancel(LiquidCancelOrderEvent),
    Match(LiquidMatchOrderEvent),
}
#[derive(Debug)]
enum BinanceOrderEvent {
    Create(BinanceCreateOrderEvent),
    Cancel(BinanceCancelOrderEvent),
    Match(BinanceMatchOrderEvent),
}
#[derive(Debug)]
enum CoinbaseOrderEvent {
    Create(CoinbaseCreateOrderEvent),
    Cancel(CoinbaseCancelOrderEvent),
    Match(CoinbaseMatchOrderEvent),
}
#[derive(Debug)]
enum InternalOrderEvent {
    Create(InternalCreateOrderEvent),
    Cancel(InternalCancelOrderEvent),
    Match(InternalMatchOrderEvent),
}

trait OrderEventVisitor<Input> {
    //type Input: OrderEventSet;
    //type Output: OrderEventSet;
    type Output;
    fn visit_create_order(
        &self,
        //e: &<<Self as OrderEventVisitor>::Input as OrderEventSet>::CreateOrderEvent,
        _e: &Input,
        //) -> <<Self as OrderEventVisitor>::Output as OrderEventSet>::CreateOrderEvent;
    ) -> Option<Self::Output> {
        None
    }
    fn visit_cancel_order(
        &self,
        //e: &<<Self as OrderEventVisitor>::Input as OrderEventSet>::CancelOrderEvent,
        _e: &Input,
        //) -> <<Self as OrderEventVisitor>::Output as OrderEventSet>::CancelOrderEvent;
    ) -> Option<Self::Output> {
        None
    }
    fn visit_match_order(
        &self,
        //e: &<<Self as OrderEventVisitor>::Input as OrderEventSet>::MatchOrderEvent,
        _e: &Input,
        //) -> <<Self as OrderEventVisitor>::Output as OrderEventSet>::MatchOrderEvent;
    ) -> Option<Self::Output> {
        None
    }
}

struct LiquidOrderEventVisitor;
//impl OrderEventVisitor<LiquidOrderEventSet, InternalOrderEventSet> for LiquidOrderEventVisitor {
impl OrderEventVisitor<LiquidOrderEvent> for LiquidOrderEventVisitor {
    //type Input = LiquidOrderEventSet;
    //type Output = InternalOrderEventSet;
    type Output = InternalOrderEvent;
    //fn visit_create_order(&self, e: &LiquidCreateOrderEvent) -> InternalCreateOrderEvent {
    fn visit_create_order(&self, e: &LiquidOrderEvent) -> Option<InternalOrderEvent> {
        match e {
            LiquidOrderEvent::Create(x) => {
                Some(InternalOrderEvent::Create(InternalCreateOrderEvent(x.0)))
            }
            _ => None,
        }
    }
    fn visit_cancel_order(&self, e: &LiquidOrderEvent) -> Option<InternalOrderEvent> {
        match e {
            LiquidOrderEvent::Cancel(x) => {
                Some(InternalOrderEvent::Cancel(InternalCancelOrderEvent(x.0)))
            }
            _ => None,
        }
    }
    fn visit_match_order(&self, e: &LiquidOrderEvent) -> Option<InternalOrderEvent> {
        match e {
            LiquidOrderEvent::Match(x) => {
                Some(InternalOrderEvent::Match(InternalMatchOrderEvent(x.0)))
            }
            _ => None,
        }
    }
}
struct BinanceOrderEventVisitor;
//impl OrderEventVisitor<BinanceOrderEventSet, InternalOrderEventSet> for BinanceOrderEventVisitor {
impl OrderEventVisitor<BinanceOrderEvent> for BinanceOrderEventVisitor {
    //type Input = BinanceOrderEventSet;
    type Output = InternalOrderEvent;
    fn visit_create_order(&self, e: &BinanceOrderEvent) -> Option<InternalOrderEvent> {
        match e {
            BinanceOrderEvent::Create(x) => {
                Some(InternalOrderEvent::Create(InternalCreateOrderEvent(x.0)))
            }
            _ => None,
        }
    }
    fn visit_cancel_order(&self, e: &BinanceOrderEvent) -> Option<InternalOrderEvent> {
        match e {
            BinanceOrderEvent::Cancel(x) => {
                Some(InternalOrderEvent::Cancel(InternalCancelOrderEvent(x.0)))
            }
            _ => None,
        }
    }
    fn visit_match_order(&self, e: &BinanceOrderEvent) -> Option<InternalOrderEvent> {
        match e {
            BinanceOrderEvent::Match(x) => {
                Some(InternalOrderEvent::Match(InternalMatchOrderEvent(x.0)))
            }
            _ => None,
        }
    }
}
struct CoinbaseOrderEventVisitor;
impl OrderEventVisitor<CoinbaseOrderEvent> for CoinbaseOrderEventVisitor {
    type Output = InternalOrderEvent;
    fn visit_create_order(&self, e: &CoinbaseOrderEvent) -> Option<InternalOrderEvent> {
        match e {
            CoinbaseOrderEvent::Create(x) => {
                Some(InternalOrderEvent::Create(InternalCreateOrderEvent(x.0)))
            }
            _ => None,
        }
    }
    fn visit_cancel_order(&self, e: &CoinbaseOrderEvent) -> Option<InternalOrderEvent> {
        match e {
            CoinbaseOrderEvent::Cancel(x) => {
                Some(InternalOrderEvent::Cancel(InternalCancelOrderEvent(x.0)))
            }
            _ => None,
        }
    }
    fn visit_match_order(&self, e: &CoinbaseOrderEvent) -> Option<InternalOrderEvent> {
        match e {
            CoinbaseOrderEvent::Match(x) => {
                Some(InternalOrderEvent::Match(InternalMatchOrderEvent(x.0)))
            }
            _ => None,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let loev = LiquidOrderEventVisitor;
    let boev = BinanceOrderEventVisitor;
    let coev = CoinbaseOrderEventVisitor;

    // let liquid_events: Vec<Box<dyn OrderEventVisitee>> = vec![
    //     LiquidCreateOrderEvent(1),
    //     LiquidCancelOrderEvent(1),
    //     LiquidMatchOrderEvent(1),
    // ];
    // let binance_events: Vec<Box<dyn OrderEventVisitee>> = vec![
    //     BinanceCreateOrderEvent(1),
    //     BinanceCancelOrderEvent(1),
    //     BinanceMatchOrderEvent(1),
    // ];
    // let coinbase_events: Vec<Box<dyn OrderEventVisitee>> = vec![
    //     CoinbaseCreateOrderEvent(1),
    //     CoinbaseCancelOrderEvent(1),
    //     CoinbaseMatchOrderEvent(1),
    // ];

    println!("loev: {:?}", LiquidCreateOrderEvent(1).accept(&loev));
    println!("loev: {:?}", LiquidCancelOrderEvent(1).accept(&loev));
    println!("loev: {:?}", LiquidMatchOrderEvent(1).accept(&loev));

    println!("boev: {:?}", BinanceCreateOrderEvent(2).accept(&boev));
    println!("boev: {:?}", BinanceCancelOrderEvent(2).accept(&boev));
    println!("boev: {:?}", BinanceMatchOrderEvent(2).accept(&boev));

    println!("boev: {:?}", CoinbaseCreateOrderEvent(3).accept(&coev));
    println!("boev: {:?}", CoinbaseCancelOrderEvent(3).accept(&coev));
    println!("boev: {:?}", CoinbaseMatchOrderEvent(3).accept(&coev));

    Ok(())
}
