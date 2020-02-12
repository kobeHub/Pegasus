use sparkpost::transmission::{EmailAddress, Message,
                              Options, Recipient, Transmission,
                              TransmissionResponse};

use crate::errors::ServiceError;
use crate::models::invitation::Invitation;
use crate::utils::{SPARKPOST_API_KEY, SENDING_EMAIL_ADDRESS,
                   ORGANISE_NAME};

pub fn send_invitation(invit: &Invitation) -> Result<(), ServiceError> {
    let tm = Transmission::new(SPARKPOST_API_KEY.as_str());

    let mut email = Message::new(EmailAddress::new(SENDING_EMAIL_ADDRESS, ORGANISE_NAME));

    let opts = Options {
        open_tracking: false,
        click_tracking: false,
        transactional: true,
        sandbox: false,
        inline_css: false,
        start_time: None,
    };

    // The recipient of the email
    let rec: Recipient = invit.email.as_str().into();

    // TODO: hard-code to be fixed
    let email_body = format!(
        "Please click on the link below to complete registration. <br/>
         <a href=\"http://localhost:8088/register.html?id={}&email={}\">
         http://localhost:3030/register</a> <br>
         your Invitation expires on <strong>{}</strong>",
        invit.id,
        invit.email,
        invit.expires_at
            .format("%I:%M %p %A, %-d %B, %C%y")
            .to_string()
    );

    email.add_recipient(rec)
        .options(opts)
        .subject("You have been invited to join Pegasus")
        .html(email_body);

    let result = tm.send(&email);

    match result {
        Ok(res) => match res {
            TransmissionResponse::ApiResponse(api_res) => {
                dbg!("API Response: \n {:#?}", api_res);
                Ok(())
            }
            TransmissionResponse::ApiError(errors) => {
                dbg!("Response Errors: \n {:#?}", &errors);
                Err(ServiceError::InternalServerError)
            }
        },
        Err(error) => {
            println!("Send Email Error: \n {:#?}", error);
            Err(ServiceError::InternalServerError)
        }
    }
}
