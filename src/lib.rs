//! [`GmailnatorInbox`]: struct.GmailnatorInbox.html
//! [`MailMessage`]: struct.MailMessage.html
//! This library contains objects to create a gmailnator inbox and read the messages it contains.
//! # Getting started : 
//! The main struct is the [`GmailnatorInbox`] struct, one instance contains one inbox associated to an email address.
//! 
//! This creates a new temporary gmail address :
//! ```
//! use gmailnator::GmailnatorInbox;
//! 
//! let inbox = GmailnatorInbox::new().unwrap();
//! ```
//! 
//! To get the associated mail address :
//! ```
//! # use gmailnator::GmailnatorInbox;
//! # let inbox = GmailnatorInbox::new().unwrap();
//! let address:&str = inbox.get_address();
//! ```
//! 
//! This creates `n` number of addresses, it must be used to create a large number of inboxes.
//! ```
//! use gmailnator::GmailnatorInbox;
//!
//! let n:u32 = 500;
//! let inboxes:Vec<GmailnatorInbox> = GmailnatorInbox::new_bulk(n).unwrap();
//! 
//! assert_eq!(inboxes.len() as u32, n); 
//! ```
//! 
//! To retrieve messages and display them via the container struct [`MailMessage`]: 
//! ```
//! use gmailnator::{GmailnatorInbox, MailMessage};
//! # let inbox = GmailnatorInbox::new().unwrap();
//! let messages:Vec<MailMessage> = inbox.get_messages().expect("Failed to retrieve messages.");
//! 
//! for message in messages {
//! 
//!     let title = message.get_subject();
//! 
//!     let raw_body = message.get_raw_content();
//!     let decoded_body = message.decode_content().unwrap();
//! 
//!     // raw_body     = &lt;You&gt; Where did you put the &quot;thing&quot; ?
//!     // decoded_body = <You> Where did you put the "thing" ?
//! 
//!     println!("Title : {}\nBody : {}", title, decoded_body);
//! 
//! }
//! ```

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

#[macro_use]
extern crate lazy_static;

mod errors;
mod mail;
mod endpoint;
mod regexes;
mod http;

pub use mail::{MailMessage, GmailnatorInbox, Error};
pub use errors::GmailnatorError;

#[cfg(test)]
mod tests {

    use crate::mail::{GmailnatorInbox, MailMessage};

    #[test]
    fn create_inbox() {

        let inbox = GmailnatorInbox::new().expect("Failed to create an inbox."); 

        let address = inbox.get_address();

        println!("Inbox created with email : {}", address);

        assert!(address.contains('@'))

    }

    #[test]
    fn retrieve_messages() {
        
        let inbox = GmailnatorInbox::new().expect("Failed to create an inbox."); 

        let messages = inbox.get_messages().expect("Failed to retrieve messages.");

        assert_eq!(messages.len(), 0);

    }

    #[test]
    fn create_inbox_from_existing_address() {

        let new_address = GmailnatorInbox::new().unwrap();
        let new_address = new_address.get_address();
        
        let inbox = GmailnatorInbox::from_address(new_address).unwrap();

        assert_eq!(inbox.get_address(), new_address);

    }  

    #[test]
    fn create_bulk() {
        
        let count:u32 = 1;

        let inboxes = GmailnatorInbox::new_bulk(count).unwrap();   

        assert_eq!(inboxes.len() as u32, count);

    }

    #[test]
    fn create_bulk_larger() {
        
        let count:u32 = 1000;

        let inboxes = GmailnatorInbox::new_bulk(count).unwrap();   

        assert_eq!(inboxes.len() as u32, count);

    }

    #[test]
    fn create_bulk_invalid() {

        assert!(GmailnatorInbox::new_bulk(0).is_err())

    }

    #[test]
    fn parse_mail_message_classic() {

        let message = MailMessage::parse("<b>subject</b><div>an eternity ago</div><hr/><div dir=\"ltr\">content</div>").unwrap();

        assert_eq!(message.get_subject(), "subject");

        assert_eq!(message.decode_content().unwrap(), "content");
        assert_eq!(message.get_raw_content(), "content");

    }

    #[test]
    fn parse_mail_message_encoded() {

        let confirmation_html_body = "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Strict//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd\">
        <html>
        <head>
          <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=0\" />
          <meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\">
          <title>No-IP - Free Dynamic DNS - Managed DNS - Managed Email - Domain Registration</title>
          <meta charset=\"utf-8\">
          <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
          <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\" />
          <link href'https://fonts.googleapis.com/css?familyOpen+Sans:400,300=
          ,300italic,400italic,600,600italic,700italic,800,800italic,700' rel'styl=
          esheet' type'text/css'>
          <style type=\"text/css\">
              /* CLIENT-SPECIFIC STYLES */
              body, table, td, a{-webkit-text-size-adjust: 100%; -ms-text-size-adjust: 100%; font-family: 'Open Sans', Helvetica, Arial, sans-serif; line-height: 25px;} /* Prevent WebKit and Windows mobile changing default text sizes */
              table, td{mso-table-lspace: 0pt; mso-table-rspace: 0pt;} /* Remove spacing between tables in Outlook 2007 and up */
              img{-ms-interpolation-mode: bicubic;} /* Allow smoother rendering of resized image in Internet Explorer */
        
              /* RESET STYLES */
              img{border: 0; height: auto; line-height: 100%; outline: none; text-decoration: none;}
              table{border-collapse: collapse !important;}
              body{height: 100% !important; margin: 0 !important; padding: 0 !important; width: 100% !important;}
              .padding-copy { padding: 5px 20px !important; text-align: left;}
              .padding-top { padding: 40px 40px!important; text-align: left;}
              .padding-bottom { padding: 0 20px 20px !important; text-align: left;}
        
              /* iOS BLUE LINKS */
              a[x-apple-data-detectors] {
                  color: inherit !important;
                  text-decoration: none !important;
                  font-size: inherit !important;
                  font-family: inherit !important;
                  font-weight: inherit !important;
                  line-height: inherit !important;
              }
        
              /* MOBILE STYLES */
              @media  screen and (max-width: 525px) {
        
                  /* ALLOWS FOR FLUID TABLES */
                  .wrapper {
                    width: 100% !important;
                      max-width: 100% !important;
                  }
        
                  /* ADJUSTS LAYOUT OF LOGO IMAGE */
                  .logo img {
                    margin: 0 auto !important;
                  }
        
                  /* USE THESE CLASSES TO HIDE CONTENT ON MOBILE */
                  .mobile-hide {
                    display: none !important;
                  }
        
                  .img-max {
                    max-width: 100% !important;
                    width: 100% !important;
                    height: auto !important;
                  }
        
                  /* FULL-WIDTH TABLES */
                  .responsive-table {
                    width: 100% !important;
                  }
        
                  /* UTILITY CLASSES FOR ADJUSTING PADDING ON MOBILE */
                  .padding {
                    padding: 10px 5% 15px 5% !important;
                  }
        
                  .padding-meta {
                    padding: 30px 5% 0px 5% !important;
                    text-align: left;
                  }
        
                  .padding-copy {
                   padding: 5px 20px !important;
                    text-align: left;
                  }
        
                  .no-padding {
                    padding: 0 !important;
                  }
        
                  .section-padding {
                    padding: 50px 15px 50px 15px !important;
                  }
        
                  /* ADJUST BUTTONS ON MOBILE */
                  .mobile-button-container {
                      margin: 0 auto;
                      width: 100% !important;
                  }
        
                  .mobile-button {
                      padding: 15px !important;
                      border: 0 !important;
                      font-size: 16px !important;
                      display: block !important;
                  }
        
                  .logo {
                      padding-left: 0px;
                  }
        
              }
        
              /* ANDROID CENTER FIX */
              div[style*=\"margin: 16px 0;\"] { margin: 0 !important; }
          </style>
        
          </head>
          <body style=\"margin: 0 !important; padding: 0 !important; background-color: #f9f9f9;\">
        
          <!-- HEADER -->
          <table border=\"0\" cellpadding=\"0\" cellspacing=\"0\" width=\"100%\" align=\"center\">
              <tr>
                  <td align=\"center\">
                      <!--[if (gte mso 9)|(IE)]>
                      <table align=\"center\" border=\"0\" cellspacing=\"0\" cellpadding=\"0\" width=\"500\">
                      <tr>
                      <td align=\"center\" valign=\"top\" width=\"500\">
                      <![endif]-->
                      <table border=\"0\" cellpadding=\"0\" cellspacing=\"0\" width=\"100%\" style=\"max-width: 600px;\" class=\"wrapper\">
                          <tr>
                              <td align=\"left\" valign=\"top\" style=\"padding: 30px 0;\" class=\"logo\">
                                                      <a href=\"https://url9788.noip.com/ls/click?upn=LUi80JKtjcz7uPXfjVJj8Jh0GJvZ6j5N2-2FYhDZh9cVDadrUq5irg8wXE7vDyw7GXGvkmODUcP9F2VgoLWYMTKz-2F7NGCSYwYZJD7MakUwtdnHSdbV8Yf3rjsbiTlvM2JHR05B_oBO4hZ5VW1W5oxnrZqjkD4gdhEcULBt4ZZpyJKV3x52NmLTJTPWxcaL8QX6kSVJdnLpzinL6-2FIiBfHehwJGUveXV-2BtOfVMy4-2BAuqZSvJYsnu1ljPRos-2BD2MhREBTnBZYeGcPTH573RjVTPW6fMfaiJnEdp-2B-2B5JD-2FPSDrj1TUu0GSeRwZoPuUMf2b4wwfJgvv5b0bzeE0pYDy-2FBsjy9QNICw-2BtttDobQTW0ORI-2Fbx-2BsU0-2BEjx1iZCryRaG0cmSg7fQKFA-2FXmkTteqM4PDbSd10-2By8d97I5DeLZVcZpsIsxEMATDMt59FqwrgSpu3lA-2BTg\">
                                                    <img src=\"https://d3ezh8unnmkdru.cloudfront.net/img/logo-grey@2x.png\" style=\"outline: none; border: 0;\"/>
                            </a>
                          </td>
                      </tr>
                  </table>
                  <!--[if (gte mso 9)|(IE)]>
                  </td>
                  </tr>
                  </table>
                  <![endif]-->
              </td>
          </tr>
          <tr>
              <td  align=\"center\" style=\"padding: 15px;\">
                  <!--[if (gte mso 9)|(IE)]>
                  <table align=\"center\" border=\"0\" cellspacing=\"0\" cellpadding=\"0\" width=\"500\">
                  <tr>
                  <td align=\"center\" valign=\"top\" width=\"500\">
                  <![endif]-->
                  <table bgcolor=\"#ffffff\" border=\"0\" cellpadding=\"0\" cellspacing=\"0\" width=\"100%\" style=\"max-width: 600px; padding: 20px; padding-top: 20px;\" class=\"responsive-table\">
                      <tr>
                          <td>
                              <!-- COPY -->
                              <table width=\"100%\" style=\"max-width: 600px;\" border=\"0\" cellspacing=\"0\" cellpadding=\"0\">
                                  <tr>
                                    <td align=\"left\" style=\"font-size: 14px; font-family: 'Open Sans', Helvetica, Arial, sans-serif; color: #666666; padding-top: 30px;\" class=\"padding-top\">
                                        <span style=\"font-weight:300; font-size:24px; letter-spacing:0.025em; line-height:23px; color:#8FBE00; font-family:'Poppins', sans-serif; mso-line-height-rule: exactly;\">
          Confirm Your No-IP Account<br/>
        </span>
        
          <p>Thanks for creating a No-IP account. We are happy you found us. To confirm your account, please click the button below.</p>
        
          <table class=\"table-button180\" height=\"45\" align=\"center\" cellpadding=\"0\" cellspacing=\"0\" border=\"0\" bgcolor=\"#8FBE00\" style=\"margin: 0 auto; border-radius: 3px; min-width: 220px;\">
          <tr>
            <td align=\"center\" valign=\"middle\" style=\"padding: 5px 15px\">
                      <a href=\"https://url9788.noip.com/ls/click?upn=LUi80JKtjcz7uPXfjVJj8Cdtxl9NGIH2j18Fo43xwJBK6C00sv4cuxbtusnfBcwJSShFfCi4MjeXMeXQ-2BCk1hFBQOyk-2FR1BBwKUI4QeS3hqOKa19uiOaIVh6hmDaV5x99ttGSIDAa9vHmT-2FMsc1zqT09ccMXHbB8-2FSBb-2Bs-2Fwock-3Dbpyw_oBO4hZ5VW1W5oxnrZqjkD4gdhEcULBt4ZZpyJKV3x52NmLTJTPWxcaL8QX6kSVJdnLpzinL6-2FIiBfHehwJGUvc6T0X-2BIFkZtEoSx8SX-2FmQt-2BzGZkcb7dRSFVH-2FHvw9dfyDJCXm2gqYcHkhH4s23RzlFC3BsyVO-2FAgAMALxtpumK7fgHkL08MGMFDotvrM4hBPZqD6UVy0VqBRUK-2FiHkcNj-2FTEoMV61vVr-2BQy3-2FuYBJELTMQh65KhtcYBnku2xNKO1lnOO7u3NMvE5b3s0v1AbsbB4q70Ket9htIfBoPMRTwyJ7uPLdO3fa55tLL2FWQo\" style=\"font-weight:500; font-size:17px; letter-spacing:0.025em; line-height:26px; color:#FFF;font-family:'Poppins', sans-serif; mso-line-height-rule: exactly; text-decoration:none;\">
                      Confirm Account
              </a>
            </td>
          </tr>
        </table>
        
          <p>Need help? Open a <a href=\"https://url9788.noip.com/ls/click?upn=DZBbzLq10FOlNC8hVdX-2Bw-2B13kIjEWxD70gMxHgjR7V2WMgl6kCDNEkPiSmxoMtkEEqVDQjlQ7ASrmo7Yr-2BFR1MjLZNi1NWi7oPc66XorPRmL2mCiEoxA6YybDZnS8zVoYc3wkf76Kl07gsLZFPi-2FgA-3D-3DHvvu_oBO4hZ5VW1W5oxnrZqjkD4gdhEcULBt4ZZpyJKV3x52NmLTJTPWxcaL8QX6kSVJdnLpzinL6-2FIiBfHehwJGUvfmGYP289tLFwV3z5FyGK8c7R7j65uUowkKS0figX6GsP5d5TofvbD6-2Fn1-2BLZf6KO5fAAdFlsPOhqHIxQP0Hv8PwMxBeoIRpvQol6BLmpyCFv-2FKDIacN3amm3SPtKnOyEAZ6e6ySSl4gMl1bnV-2ByonKJf995WjiVjqCbs7n6cQLZD9aqH7SJpRvEXgX3AzSBVieTRP5an26F3YoQAM-2BIx7E-2FKMG9NipbFNhKFVwRzd0B\" style=\"color: #2A9AC9\">Support Ticket</a>
        
         now.</p>
        
          <p>Thank you for choosing No-IP! We hope that you enjoy our rock solid services that we have been offering since 1999 to millions of users.</p>
        
                                    </td>
                                </tr>
                            </table>
                        </td>
                    </tr>
                </table>
                <!--[if (gte mso 9)|(IE)]>
                </td>
                </tr>
                </table>
                <![endif]-->
            </td>
        </tr>
        <tr>
            <td bgcolor=\"#f9f9f9\" align=\"center\" style=\"padding: 20px 0px;\">
                <!--[if (gte mso 9)|(IE)]>
                <table align=\"center\" border=\"0\" cellspacing=\"0\" cellpadding=\"0\" width=\"500\">
                <tr>
                <td align=\"center\" valign=\"top\" width=\"500\">
                <![endif]-->
                <!-- UNSUBSCRIBE COPY -->
                <table width=\"100%\" border=\"0\" cellspacing=\"0\" cellpadding=\"0\" align=\"center\" style=\"max-width: 600px;\" class=\"responsive-table\">
                    <tr>
                        <td align=\"center\" style=\"font-size: 12px; line-height: 18px; font-family: 'Open Sans', Helvetica, Arial, sans-serif; color:#666666;\">
                            Vitalwerks Internet Solutions, LLC c/o No-IP.com<br>
                                                425 Maestro Dr. Suite 200<br>
                            Reno, Nevada 89511 USA<br>
                            +1 775-853-1883
                          </td>
                        </tr>
                        <tr>
                          <!-- START left block -->
                          <td class=\"td-display-block-center-pad18\" width=\"280\" valign=\"top\" style=\"padding:18px 0px 0px 0px; font-weight:300; font-size:12px; letter-spacing:0.025em; line-height:40px; color:#666666; font-family:'Poppins', sans-serif; mso-line-height-rule: exactly;\" align=center colspan=2 >
                          &copy; 1999-2020 Vitalwerks Internet Solutions, LLC.
                          </td>
                          <!-- END left block -->
                          <!-- START right block -->
                                            <!-- END right block -->
                        </tr>
                    </table>
                    <!--[if (gte mso 9)|(IE)]>
                    </td>
                    </tr>
                    </table>
                    <![endif]-->
                </td>
            </tr>
        </table>
        <img src=\"https://url9788.noip.com/wf/open?upn=eUn-2FogS-2F7LpNeRV03ugGy-2FkcKhVPZyKYW-2BOOtMynDA-2FGkxdC-2FfQLnxU-2F-2BCzwa5-2B6zAjLC-2B-2F1GM3f9SF-2FhJcxzF-2BxNSxa0XJIH0Q5xQr29mfqlt4P433SH8RgHJoz7ywv7p-2BOEURtdnLZw-2FYWspFhUXfGq63-2BvZYH7X-2BvcTBzYvLgaSBl5wfJ8Gu-2FEv8jodFranCXYFB7mFHLGk4bNxe2KaHUMiaPMDcx-2FYzRUUNXK3TPk6-2BHGWxunP5uo2nHkZRiUjHwm5HA4I-2BJJI8pbB-2Be8ZzpcGMT6SzNGC-2BeAqgExAN0ln4ZNMv0a9cvSFYY6I4aPQfbWtbsgVhPpKMFqD34dw-3D-3D\" alt=\"\" width=\"1\" height=\"1\" border=\"0\" style=\"height:1px !important;width:1px !important;border-width:0 !important;margin-top:0 !important;margin-bottom:0 !important;margin-right:0 !important;margin-left:0 !important;padding-top:0 !important;padding-bottom:0 !important;padding-right:0 !important;padding-left:0 !important;\"/>
        </body>
        </html>";

        let fragment = format!("{}{}", "<b>subject</b><div>an eternity ago</div><hr />", confirmation_html_body);

        let message = MailMessage::parse(&fragment).unwrap();

        assert_eq!(message.get_subject(), "subject");

        assert_eq!(message.get_raw_content(), confirmation_html_body);
        
        assert_ne!(message.decode_content().unwrap(), confirmation_html_body); //Doesn't assert because there's a &copy; entity that gets decoded...

    }

}