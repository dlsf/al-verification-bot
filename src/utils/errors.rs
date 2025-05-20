use custom_error::custom_error;

custom_error!{pub AccountLinkError
    NotLinked = "The account hasn't been linked yet",
    AlreadyLinked = "You've already linked your account!",
    Discord = "Failed to contact Discord",
    Database = "An database error occurred, please try again later!",
    Anilist = "Couldn't verify your account, please get a new token and try again later!",
    AccountAge{remaining_time_hours: u32} = "Your AniList account is too new to be verified, please try again in {remaining_time_hours} hours!"
}

custom_error!{pub GraphQLError
    Request = "Received invalid response from server",
    View = "Failed to parse response from server"
}

custom_error! {pub PermissionError
    Discord = "Failed to contact Discord"
}