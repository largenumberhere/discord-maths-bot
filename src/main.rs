

use serenity::{
    Client, prelude::{
        GatewayIntents, EventHandler, Context
    }, 
    async_trait, model::prelude::{
        interaction::{
            Interaction, application_command::{
                CommandDataOption, CommandDataOptionValue
            }, InteractionResponseType
        }, 
        Ready, command::{
            Command, CommandOptionType
        }
    }
};
use shunting::{ShuntingParser, MathContext};


struct  Handler;

#[async_trait]
impl EventHandler for Handler{
    async fn interaction_create(&self, context: Context, interaction: Interaction){
        if let Interaction::ApplicationCommand(command) = interaction{
            //println!("Received interaction. {:#?}",command);
            let id = command.user.id;
            println!("Interaction received from user with id '{}'",id);

            let content = match command.data.name.as_str() {
                "maths" => maths_command(&command.data.options),
                _ => "not implemented".to_string(),             
            };

            if let Err(error) = command
                .create_interaction_response(&context.http, |response|{
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                    
                }).await
            {
                println!("reply failed! {}",error);
            }
        }
    }

    async fn ready(&self, context:Context, ready: Ready){
        println!("{} is connected",ready.user.name);

        let command_result = Command::create_global_application_command(&context.http, |command|
            {
                command.name("maths")
                    .description("calcuates someting");
                
                command.create_option(|option|{
                    option.name("formula");
                    option.description("insert a formula");
                    option.kind(CommandOptionType::String);
                    option.required(true);
                    option

                });
                
                

                command
            }
        ).await;
        
        match command_result {
            Ok(c) =>{
                println!("created the command '{}' sucessfully",c.name);    
            },
            Err(ref e) =>{
                println!("failed to create command!\n  {:#?} \nError:{}",command_result,e);
            }
        }



    }
}

#[tokio::main]
async fn main() -> Result<(),serenity::Error>{
    let token = load_discord_token().expect("could not get token from discord.txt");
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        ?;

    if let Err(message) = client.start().await{
        println!("client error: {:?}",message);
    };
    
    Ok(())
}

fn load_discord_token() -> Result<String,std::io::Error>{
    let contents = std::fs::read_to_string("discord.txt")?;
    Ok(contents)
}


//commands
#[derive(Debug)]
struct MathsParseError{}


fn maths_command(options: &[CommandDataOption] ) -> String{
    let option0 = options.get(0)
        .expect("maths command did not give a formula").
        resolved.as_ref()
        .expect("option0 did not convert to ref");

    if let CommandDataOptionValue::String(value) = option0{
        let compute_expression =  ||{
            let expression = ShuntingParser::parse_str(value)?;
            let result = MathContext::new().eval(&expression)?;
            Ok::<f64,String>(result)
        };

        let string_reply:String = match compute_expression() {
            Ok(v) =>{
                v.to_string()
            },
            Err(e) =>{format!("failed to convert because: {}",e)}
        };

        string_reply

    }else {
        "Error: invalid option type recieved".to_string()
    }
    
    


}