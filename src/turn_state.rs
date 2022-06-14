#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TurnState {
    MainMenu,
    GlobalMap,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
    NextLevel
}