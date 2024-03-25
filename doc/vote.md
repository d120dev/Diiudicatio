# Diiudicatio Vote

## Vote Types

- Yes/No Vote
    - Archiving (System Vote)
    - Readacting (System Vote)
- Multiple Options Vote
- Alternative Vote

## Vote Access

- Open
- Protected
- Closed

## Majorities

- Relative Majority: Option with the most votes winns. [^0]
- Simple Majority: Option with more votes than all other Options winns. [^0]
- Simple Qualified Majority: Option with the most votes that reach a threshold (quorum) of votes wins. [^0]
- Absolute Qualified Majority: Option with the most votes that reaches a threshold (quorum) of attendant members. [^1]
- Alternative Vote Majoritiy: see [Alternative Vote](#alternative-vote)

[^0]: Abstentions do not count as votes for an option.
[^1]: Abstentions are counted to determine if the quorum is reached.

## Alternative Vote

## Vote State Machine

![VoteStateMachine](./assets/vote_state.drawio.png)

$$
\begin{align*}
\text{States} &=\{\text{Created}, \text{Open}, \text{Accepted}, \text{Rejected}, \text{Archived}, \text{Redacted}, \text{Deleted}\} \\
\text{Initial States} &= \{\text{Created}\} \\
\text{Accepted States} &= \{\text{Accepted}, \text{Rejected}, \text{Archived}, \text{Redacted}, \text{Deleted}\} \\
\text{Alphabet} &= \{modify, open, close_{accept}, close_{reject}, archive, delete, redact\} \\
\text{Transitions} &= \lbrace \\
    & \qquad (\text{Created}, modify, \text{Created}) \\
    & \qquad (\text{Created}, delete, \text{Deleted}) \\
    & \qquad (\text{Created}, open, \text{Open}) \\
    & \qquad (\text{Open}, close_{accept}, \text{Accepted}) \\
    & \qquad (\text{Accepted}, modify, \text{Accepted}) \\
    & \qquad (\text{Accepted}, archive, \text{Archived}) \\
    & \qquad (\text{Archived}, redact, \text{Redacted}) \\
    & \qquad (\text{Open}, close_{reject}, \text{Rejected}) \\
    & \qquad (\text{Rejected}, redact, \text{Redacted}) \\
&\quad \rbrace \\
\end{align*}
$$