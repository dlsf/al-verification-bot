use custom_error::custom_error;

custom_error!{pub AccountLinkError
    NotLinked = "The account hasn't been linked yet"
}

custom_error!{pub GraphQLError
    RequestError = "Got invalid response from server",
    ViewError = "Failed to parse response from server"
}
