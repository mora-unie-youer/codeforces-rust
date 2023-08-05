use std::{collections::HashSet, io::BufRead};

/**
 * A. Chat Server's Outgoing Traffic
 * time limit per test: 1 second
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * Polycarp is working on a new project called "Polychat". Following modern tendencies in IT, he decided, that this project
 * should contain chat as well. To achieve this goal, Polycarp has spent several hours in front of his laptop and
 * implemented a chat server that can process three types of commands:
 *     Include a person to the chat ('Add' command).
 *     Remove a person from the chat ('Remove' command).
 *     Send a message from a person to all people, who are currently in the chat, including the one, who sends the message
 * ('Send' command).
 *
 * Now Polycarp wants to find out the amount of outgoing traffic that the server will produce while processing a particular
 * set of commands.
 *
 * Polycarp knows that chat server sends no traffic for 'Add' and 'Remove' commands. When 'Send' command is processed,
 * server sends l bytes to each participant of the chat, where l is the length of the message.
 *
 * As Polycarp has no time, he is asking for your help in solving this problem.
 *
 * Input
 * Input file will contain not more than 100 commands, each in its own line. No line will exceed 100 characters. Formats of
 * the commands will be the following:
 *     +<name> for 'Add' command.
 *     -<name> for 'Remove' command.
 *     <sender_name>:<message_text> for 'Send' command.
 *
 * <name> and <sender_name> is a non-empty sequence of Latin letters and digits. <message_text> can contain letters, digits
 * and spaces, but can't start or end with a space. <message_text> can be an empty line.
 *
 * It is guaranteed, that input data are correct, i.e. there will be no 'Add' command if person with such a name is already
 * in the chat, there will be no 'Remove' command if there is no person with such a name in the chat etc.
 *
 * All names are case-sensitive.
 *
 * Output
 * Print a single number — answer to the problem.
 *
 * Examples
 *   Input
 *   +Mike
 *   Mike:hello
 *   +Kate
 *   +Dmitry
 *   -Dmitry
 *   Kate:hi
 *   -Kate
 *   
 *   Output
 *   9
 *   
 *   Input
 *   +Mike
 *   -Mike
 *   +Mike
 *   Mike:Hi   I am here
 *   -Mike
 *   +Kate
 *   -Kate
 *   
 *   Output
 *   14
 */
fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let mut traffic = 0;
    let mut participants = HashSet::new();
    for line in lines {
        let line = line.unwrap();

        if let Some(person) = line.strip_prefix('+') {
            participants.insert(person.to_owned());
        } else if let Some(person) = line.strip_prefix('-') {
            participants.remove(person);
        } else {
            let (_sender, message) = line.split_once(':').unwrap();
            traffic += message.len() * participants.len();
        }
    }

    println!("{traffic}");
}
