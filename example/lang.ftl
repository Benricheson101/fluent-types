shared-photos =
    {$userName} {$photoCount ->
        [one] added a new photo
       *[other] added {$photoCount} new photos
    } to {$userGender ->
        [male] his stream
        [female] her stream
       *[other] their stream
    }.

msg-with-no-placeable = This message has no placeable elements

# message comment
msg-with-inline-select = Good morning, {$name}

greet =
    { $timeOfDay ->
        [morning] Good morning
        [evening] Good evening
        *[day] Good day
    }, { $name }!

your-rank = { NUMBER($pos, type: "ordinal") ->
    [1] You finished first!
    [one] You finished {$pos}st
    [two] You finished {$pos}nd
    [few] You finished {$pos}rd
   *[other] You finished {$pos}th
}

some-text = Hi, {$name}

multiple-occur-same-placeholder = hi {$name} hello {$name} greetings {$name}

hi =
    .hello = "hi {$name}"
    .hello1 = "hi {$name}"
    .hello2 = "hi {$name}"
    .hello3 = "hi {$name}"
    .hello4 = "hi {$name}"
    .hello5 = "hi {$name}"
    .hello6 = "hi {$name}"
    .hello7 = "hi {$name}"
    .hello8 = "hi {$name}"
    .hello9 = "hi {$name}"
    .hello10 = "hi {$name}"
    .hello11 = "hi {$name}"
    .hello12 = "hi {$name}"
    .hello13 = "hi {$name}"
    .hello14 = "hi {$name}"
    .hello15 = "hi {$name}"
