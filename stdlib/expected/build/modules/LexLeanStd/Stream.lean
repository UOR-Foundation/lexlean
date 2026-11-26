module
public import Init
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanStd.Stream

@[expose] public def appendItems (Item : Type) : (left : List (Item)) -> (right : List (Item)) -> List (Item)
  | List.nil, right => right
  | List.cons item rest, right => (item :: appendItems (Item) (rest) (right))

@[expose] public def joinChunks (Item : Type) : (chunks : List (List (Item))) -> List (Item)
  | List.nil => ([] : List (Item))
  | List.cons chunk rest => appendItems (Item) (chunk) (joinChunks (Item) (rest))

@[expose] public def feedAll (State : Type) (Item : Type) : (feed : ((State) -> (Item) -> (State))) -> (state : State) -> (items : List (Item)) -> State
  | _feed, state, List.nil => state
  | feed, state, List.cons item rest => feedAll (State) (Item) (feed) ((feed (state) (item))) (rest)

@[expose] public def feedChunks (State : Type) (Item : Type) : (feed : ((State) -> (Item) -> (State))) -> (state : State) -> (chunks : List (List (Item))) -> State
  | _feed, state, List.nil => state
  | feed, state, List.cons chunk rest => feedChunks (State) (Item) (feed) (feedAll (State) (Item) (feed) (state) (chunk)) (rest)

public theorem feed_all_append (State : Type) (Item : Type) (feed : ((State) -> (Item) -> (State))) (state : State) (left : List (Item)) (right : List (Item)) : (feedAll (State) (Item) (feed) (state) (appendItems (Item) (left) (right)) = feedAll (State) (Item) (feed) (feedAll (State) (Item) (feed) (state) (left)) (right)) := by
  induction left generalizing state with
  | nil =>
    simp only [appendItems, feedAll]
  | cons item rest hypothesis =>
    simp only [appendItems, feedAll, hypothesis]

public theorem chunking_invariance (State : Type) (Item : Type) (feed : ((State) -> (Item) -> (State))) (state : State) (chunks : List (List (Item))) : (feedChunks (State) (Item) (feed) (state) (chunks) = feedAll (State) (Item) (feed) (state) (joinChunks (Item) (chunks))) := by
  induction chunks generalizing state with
  | nil =>
    simp only [feedAll, feedChunks, joinChunks]
  | cons chunk rest hypothesis =>
    simp only [feedChunks, feed_all_append, hypothesis, joinChunks]

end LexLeanStd.Stream
