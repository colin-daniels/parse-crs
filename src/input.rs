use std::io::Write;

use pest::iterators::Pair;
use thiserror::Error;

use crate::{
    enum_token,
    pest::{Deserialize, Rule, Serialize},
    util::PairsExt,
};

enum_token! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum InputType {
        /// Contains the combined size of all request parameters. Files are excluded from the
        /// calculation. This variable can be useful, for example, to create a rule to ensure
        /// that the total size of the argument data is below a certain threshold.
        ArgsCombinedSize     = "ARGS_COMBINED_SIZE",
        /// Contains only query string parameters.
        ArgsGet              = "ARGS_GET",
        /// Contains only the names of query string parameters.
        ArgsGetNames         = "ARGS_GET_NAMES",
        /// Contains arguments from the POST body.
        ArgsPost             = "ARGS_POST",
        /// Contains only the names of request body parameters.
        ArgsPostNames        = "ARGS_POST_NAMES",
        /// ARGS_GET + ARGS_POST
        Args                 = "ARGS",
        /// ARGS_GET_NAMES + ARGS_POST_NAMES
        ArgsNames            = "ARGS_NAMES",
        /// Contains the number of milliseconds elapsed since the beginning of the current
        /// transaction.
        Duration             = "DURATION",
        /// Contains the total size of the files transported in request body. Available only on
        /// inspected multipart/form-data requests.
        FilesCombinedSize    = "FILES_COMBINED_SIZE",
        /// Contains a list of form fields that were used for file upload. Available only on
        /// inspected multipart/form-data requests.
        FilesNames           = "FILES_NAMES",
        /// Contains a collection of original file names (as they were called on the remote user's
        /// filesystem). Available only on inspected multipart/form-data requests.
        Files                = "FILES",
        /// GEO is a collection populated by the results of the last @geoLookup operator.
        /// The collection can be used to match geographical fields looked from an IP address or
        /// hostname.
        Geo                  = "GEO",
        Ip                   = "IP",
        /// This variable holds the value of the most-recently matched variable. It is similar to
        /// TX:0, but it is automatically supported by all operators and there is no need to specify
        /// the capture action.
        MatchedVar           = "MATCHED_VAR",
        /// This variable holds the full name of the variable that was matched against.
        MatchedVarName       = "MATCHED_VAR_NAME",
        /// Similar to MATCHED_VAR except that it is a collection of all matches for the current
        /// operator check.
        MatchedVars          = "MATCHED_VARS",
        /// Similar to MATCHED_VAR_NAME except that it is a collection of all matches for the
        /// current operator check.
        MatchedVarsNames     = "MATCHED_VARS_NAMES",
        /// This variable is a collection of all part headers found within the request body with
        /// Content-Type multipart/form-data. The key of each item in the collection is the name
        /// of the part in which it was found, while the value is the entire part-header
        /// line -- including both the part-header name and the part-header value.
        MultipartPartHeaders = "MULTIPART_PART_HEADERS",
        /// Contains the query string part of a request URI. The value in QUERY_STRING is always
        /// provided raw, without URL decoding taking place.
        QueryString          = "QUERY_STRING",
        /// This variable holds the IP address of the remote client.
        RemoteAddr           = "REMOTE_ADDR",
        /// Contains the name of the currently used request body processor. The possible values are
        /// URLENCODED, MULTIPART, and XML.
        ReqBodyProcessor     = "REQBODY_PROCESSOR",
        /// This variable holds just the filename part of REQUEST_FILENAME (e.g., index.php).
        RequestBasename      = "REQUEST_BASENAME",
        /// Holds the raw request body. This variable is available only if the URLENCODED request
        /// body processor was used, which will occur by default when the
        /// application/x-www-form-urlencoded content type is detected, or if the use of the
        /// URLENCODED request body parser was forced.
        ///
        /// Note: this is done by default via `"ctl:requestBodyProcessor=URLENCODED"`
        RequestBody          = "REQUEST_BODY",
        /// This variable is a collection of the names of all request cookies.
        RequestCookiesNames  = "REQUEST_COOKIES_NAMES",
        /// This variable is a collection of all of request cookies (values only).
        RequestCookies       = "REQUEST_COOKIES",
        /// This variable holds the relative request URL without the query string part
        /// (e.g., /cgi-bin/login.php).
        RequestFilename      = "REQUEST_FILENAME",
        /// This variable is a collection of the names of all of the request headers.
        RequestHeadersNames  = "REQUEST_HEADERS_NAMES",
        /// This variable can be used as either a collection of all of the request headers or can
        /// be used to inspect selected headers (by using the REQUEST_HEADERS:Header-Name syntax).
        ///
        /// Note: ModSecurity will treat multiple headers that have identical names in accordance
        /// with how the webserver treats them. For Apache this means that they will all be
        /// concatenated into a single header with a comma as the deliminator.
        RequestHeaders       = "REQUEST_HEADERS",
        /// This variable holds the complete request line sent to the server (including the request
        /// method and HTTP version information).
        RequestLine          = "REQUEST_LINE",
        /// This variable holds the request method used in the transaction.
        RequestMethod        = "REQUEST_METHOD",
        /// This variable holds the request protocol version information. (e.g., HTTP/1.1)
        RequestProtocol      = "REQUEST_PROTOCOL",
        /// This variable holds the full request URL including the query string data
        /// (e.g., /index.php?p=X). However, it will never contain a domain name, even if it was
        /// provided on the request line.
        RequestUri           = "REQUEST_URI",
        /// Same as REQUEST_URI but will contain the domain name if it was provided on the request
        /// line (e.g., http://www.example.com/index.php?p=X).
        RequestUriRaw        = "REQUEST_URI_RAW",
        /// This variable holds the data for the response body, but only when response body
        /// buffering is enabled.
        ResponseBody         = "RESPONSE_BODY",
        /// This variable holds the HTTP response status code.
        ResponseStatus       = "RESPONSE_STATUS",
        /// This is the transient transaction collection, which is used to store pieces of data,
        /// create a transaction anomaly score, and so on. The variables placed into this collection
        /// are available only until the transaction is complete.
        ///
        /// Some variable names in the TX collection are reserved and cannot be used:
        /// - TX:0: the matching value when using the @rx or @pm operator with the capture action.
        /// - TX:1-TX:9: the captured subexpression value when using the @rx operator with capturing
        ///   parens and the capture action.
        /// - TX:MSC_.*: ModSecurity processing flags.
        /// - MSC_PCRE_LIMITS_EXCEEDED: Set to nonzero if PCRE match limits are exceeded. See
        ///   SecPcreMatchLimit and SecPcreMatchLimitRecursion for more information.
        Tx                   = "TX",
        /// This variable holds the data created by mod_unique_id
        /// http://httpd.apache.org/docs/2.2/mod/mod_unique_id.html. This module provides a magic
        /// token for each request which is guaranteed to be unique across "all" requests under
        /// very specific conditions.
        UniqueId             = "UNIQUE_ID",
        /// Special collection used to interact with the XML parser. It can be used standalone as a
        /// target for the validateDTD and validateSchema operator. Otherwise, it must contain a
        /// valid XPath expression, which will then be evaluated against a previously parsed XML
        /// DOM tree.
        Xml                  = "XML",
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Input {
    pub input: InputType,
    pub selector: Selector,
}

enum_token! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum SelectorType {
        Include = "",
        Exclude = "!",
        Count = "&",
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Selector {
    None,
    Include(String),
    Exclude(String),
    Count(String),
    CountAll,
}

impl Selector {
    pub fn arg(&self) -> Option<&str> {
        match self {
            Self::None => None,
            Self::Include(s) => Some(s),
            Self::Exclude(s) => Some(s),
            Self::Count(s) => Some(s),
            Self::CountAll => None,
        }
    }

    pub fn type_(&self) -> Option<SelectorType> {
        match self {
            Self::None => None,
            Self::Include(_) => Some(SelectorType::Include),
            Self::Exclude(_) => Some(SelectorType::Exclude),
            Self::Count(_) => Some(SelectorType::Count),
            Self::CountAll => Some(SelectorType::Count),
        }
    }

    pub fn prefix(&self) -> Option<&'static str> {
        let type_ = self.type_();
        type_.map(|t| t.name())
    }

    fn from_parts(
        modifier: Option<&str>,
        selector: Option<&str>,
        input: &str,
    ) -> Result<Selector, InputParseError> {
        use InputParseError::*;
        use SelectorType::*;

        let type_ = modifier.and_then(|m| SelectorType::from_name(m));
        Ok(match (type_, selector) {
            (Some(Exclude), Some(s)) => Selector::Exclude(s.into()),
            (Some(Exclude), None) => Err(InvalidSelector(input.into()))?,
            (Some(Count), Some(s)) => Selector::Count(s.into()),
            (Some(Count), None) => Selector::CountAll,
            (None, Some(s)) => Selector::Include(s.into()),
            (None, None) => Selector::None,
            (Some(_), _) => Err(InvalidModifier(input.into()))?,
        })
    }
}

#[derive(Debug, Clone, Error)]
pub enum InputParseError {
    #[error("invalid rule {0:?}")]
    UnexpectedRule(Rule),
    #[error("unknown input {0}")]
    UnknownInput(String),
    #[error("invalid input selector {0}")]
    InvalidSelector(String),
    #[error("invalid input modifier {0}")]
    InvalidModifier(String),
}

impl Deserialize for Input {
    const RULE: Rule = Rule::input;
    type Error = InputParseError;

    fn deserialize(input: Pair<Rule>) -> Result<Self, Self::Error> {
        if input.as_rule() != Self::RULE {
            return Err(InputParseError::UnexpectedRule(input.as_rule()));
        }

        let input_str = input.as_str();
        let mut inner = input.into_inner();

        let modifier = inner.next_if_rule(Rule::input_modifier).map(|p| p.as_str());
        let input = inner.next().unwrap().as_str();
        let input = InputType::from_name(input)
            .ok_or_else(|| InputParseError::UnknownInput(input.to_owned()))?;

        let selector = inner.next().map(|p| p.as_str());
        let selector = Selector::from_parts(modifier, selector, input_str)?;

        Ok(Self { input, selector })
    }
}

impl Serialize for Input {
    type Error = std::io::Error;

    fn serialize<W: Write>(&self, mut out: W) -> Result<(), Self::Error> {
        let input = self.input.name();
        let modifier = self.selector.prefix();
        let selector = self.selector.arg();

        match (modifier, selector) {
            (Some(m), Some(s)) => write!(out, "{m}{input}:{s}"),
            (Some(m), None) => write!(out, "{m}{input}"),
            (None, Some(s)) => write!(out, "{input}:{s}"),
            (None, None) => write!(out, "{input}"),
        }
    }
}

impl Deserialize for Vec<Input> {
    const RULE: Rule = Rule::inputs;
    type Error = InputParseError;

    fn deserialize(input: Pair<Rule>) -> Result<Self, Self::Error> {
        if input.as_rule() != Self::RULE {
            return Err(InputParseError::UnexpectedRule(input.as_rule()));
        }

        let mut actions = Vec::new();
        for action in input.into_inner() {
            actions.push(Input::deserialize(action)?);
        }

        Ok(actions)
    }
}

impl Serialize for Vec<Input> {
    type Error = std::io::Error;

    fn serialize<W: Write>(&self, mut out: W) -> Result<(), Self::Error> {
        let mut first = true;
        for input in self {
            if first {
                first = false;
            } else {
                write!(out, "|")?;
            }
            input.serialize(&mut out)?;
        }
        Ok(())
    }
}
