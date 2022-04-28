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
