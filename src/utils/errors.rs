use custom_error::custom_error;

custom_error!{pub AccountLinkError
    NotLinked = "The account hasn't been linked yet",
    DiscordError = "Failed to contact Discord"
}

custom_error!{pub GraphQLError
    RequestError = "Received invalid response from server",
    ViewError = "Failed to parse response from server"
}

custom_error! {pub PermissionError
    DiscordError = "Failed to contact Discord"}