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

# One hash
# another one hash
## Two hashes
### Three hashes
msg-with-inline-select = Good morning, {$name}
