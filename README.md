# mix_economy
A research project to mix-regulate economy in MMO worlds

### Blog posts

- [2017-10-11 Is it time to think differently about money?](https://github.com/advancedresearch/advancedresearch.github.io/blob/master/blog/2017-10-10-is-it-time-to-think-differently-about-money.md)
- [2016-10-25 Solving Economic Inequality in MMOs](http://blog.piston.rs/2016/10/25/solving-economic-inequality-in-mmos/)

[Why mix_economy?](https://github.com/PistonDevelopers/mix_economy/issues/1)

A [virtual economy](https://en.wikipedia.org/wiki/Virtual_economy)
is an emergent economy existing in a virtual persistent world,
usually exchanging virtual goods in the context of an Internet game.

One challenge is to balance gameplay to be fun for both causual and
experienced players, which most small to medium MMO games do not have
substantial resources to do.

This research project studies a simple model that can be mixed with
an existing economy model:

1. A normalized thresold sets a soft limit of the wealth of a player.
2. Money "burns in the pockets" of rich players, encouraging spending.
3. Controlled inflation charged by "total lack of money".
4. Rewards weighted as negative tax on fortune (more money = more rewards).

At start of joining the game, each player gets a start fortune.
Each player receives money rewards at regular time intervals.

The rewards increase with the amount of money, meaning that saving or
earning money is beneficial to the player.

Rewards are charged through the lack of money in circulation,
by summing the difference from the soft limit of wealth.
This means inviting new players to join is beneficial for all players.

### Text example 1

A game has an infinite resource reserve that requires a minimum amount
of time to mine. By buying an expensive equipment, the resource can be
mined faster. A player owning the expensive equipment can sell the resource
to other players, for a cheaper price than these players
earn during the same amount of time required to mine the resource.
This means a huge cash flow from
many players to some few players, which they then invest in other equipment
or spend on other goods or services to keep the value of that cash flow
from disappearing.

### Text example 2

A player want to take on a mission that takes a long time.
The reward for this mission could be an expensive equipment.
By living off the saved fortune and regular rewards, the player can
take on the mission without worrying about running out of money.

Other players can provide support with risk for themselves,
even if they do not get the reward directly, but in order to reduce
the overall cost of a resource.

### Text example 4

Items could be "programmed" to allow greater flexibility.
This will lead the players to find creative ways to make money.

## Goals

- Develop an economic model that makes gameplay fun for both experienced
  and casual players
- Make it work for "complex" environments
- Research algorithmic properties and interaction with gameplay

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
