fn are_you_playing_banjo(name: &str) -> String {


    if name.starts_with('r') || name.starts_with('R')
    {
        name.to_string() + " plays banjo"
    }
    else {
        name.to_string() + " does not play banjo"
    }

}