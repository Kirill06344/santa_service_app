

# Santa Service App
## Used technology stack
* Web Server with using Rust
    * Actix
    * Actix-web
    * Diesel
* Data base
    * Postgres
* Console Application
    * Tokio

## Tasks of project 
<table>
<tr>
    <td align="center">Task</td>
    <td align="center">Is completed</td>
</tr>
<tr>
    <td>When connecting to the service, the user uses the name. Authentication is not required.</td>
    <td style="color:#00FF00;" width="100px" align="center"><img src="images/Selected.svg" width="40px" alt=""/><br /></td>
</tr>
<tr>
    <td>Users can create groups.</td>
    <td style="color:#00FF00;" width="100px" align="center"><img src="images/Selected.svg" width="40px" alt=""/><br /></td>
</tr>
<tr>
    <td>Users can join groups.</td>
    <td style="color:#00FF00;" width="100px" align="center"><img src="images/Selected.svg" width="40px" alt=""/><br /></td>
</tr>
<tr>
    <td>Users can have administrator rights in the group.
    </td>
    <td style="color:#00FF00;" width="100px" align="center"><img src="images/Selected.svg" width="40px" alt=""/><br /></td>
</tr>
<tr>
    <td>The user who created the group automatically becomes an administrator.</td>
    <td style="color:#00FF00;" width="100px" align="center"><img src="images/Selected.svg" width="40px" alt=""/><br /></td>
</tr>
<tr>
    <td>The administrator can assign another user in the group as an administrator.</td>
    <td style="color:#00FF00;" width="100px" align="center"><img src="images/Selected.svg" width="40px" alt=""/><br /></td>
</tr>
<tr>
    <td>The administrator can remove the administrator's authority if there is at least 1 more administrator in the group.</td>
    <td style="color:#00FF00;" width="100px" align="center"><img src="images/Selected.svg" width="40px" alt=""/><br /></td>
</tr>
<tr>
    <td>The administrator can leave the group only if there is at least 1 more administrator in the group.</td>
    <td style="color:#00FF00;" width="100px" align="center"><img src="images/Selected.svg" width="40px" alt=""/><br /></td>
</tr>
<tr>
    <td>The administrator can delete the group.</td>
    <td style="color:#00FF00;" width="100px" align="center"><img src="images/Selected.svg" width="40px" alt=""/><br /></td>
</tr>
<tr>
    <td>The administrator can give a command and the service will assign a secret Santa for each member of the group, choosing from the rest of the group members.</td>
    <td style="color:#00FF00;" width="100px" align="center"><img src="images/Selected.svg" width="40px" alt=""/><br /></td>
</tr>
<tr>
    <td>Each member of the group will be assigned a secret Santa strictly to one other member of the group.</td>
    <td style="color:#00FF00;" width="100px" align="center"><img src="images/Selected.svg" width="40px" alt=""/><br /></td>
</tr>
<tr>
    <td>After that, the group becomes closed, you cannot enter or exit it.</td>
    <td style="color:#00FF00;" width="100px" align="center"><img src="images/Selected.svg" width="40px" alt=""/><br /></td>
</tr>
<tr>
    <td>Users can request for whom in the group they have become a secret Santa.</td>
    <td style="color:#00FF00;" width="100px" align="center"><img src="images/Selected.svg" width="40px" alt=""/><br /></td>
</tr>
<tr>
    <td>Use a database to store data about users, groups, and secret Santas. But you can do with storing data in memory.</td>
    <td style="color:#00FF00;" width="100px" align="center"><img src="images/Selected.svg" width="40px" alt=""/><br /></td>
</tr>
<tr>
    <td>Works via HTTP REST with JSON messages.</td>
    <td style="color:#00FF00;" width="100px" align="center"><img src="images/Selected.svg" width="40px" alt=""/><br /></td>
</tr>
<tr>
    <td>Utility for communicating with the service.</td>
    <td style="color:#00FF00;" width="100px" align="center"><img src="images/Selected.svg" width="40px" alt=""/><br /></td>
</tr>
</table>

## Contributor's list:
<table>
    <tr><td colspan="7" align="center"><b>3530904/10004<b></td></tr>
  <tr>
    <td align="center"><a href="https://github.com/Kirill06344"><img src="https://avatars.githubusercontent.com/u/67016214?v=4" width="100px" alt=""/><br /><sub><b>Bazhenov Kiril </b></sub></a><br />
    <td align="center"><a href="https://github.com/llav3ji2019"><img src="https://avatars.githubusercontent.com/u/56979109?v=4" width="100px" alt=""/><br /><sub><b>Emelyanov Pavel</b></sub></a><br />
    <td align="center"><a href="https://github.com/Koteron"><img src="https://avatars.githubusercontent.com/u/121894826?v=4" width="100px;" alt=""/><br /><sub><b>Nefedev Viktor</b></sub></a><br />
    <td align="center"><a href="https://github.com/sonix14"><img src="https://avatars.githubusercontent.com/u/117933964?v=4" width="100px;" alt=""/><br /><sub><b>Roletskaya Sofia</b></sub></a><br />
    <td align="center"><a href="https://github.com/anutatesl"><img src="https://avatars.githubusercontent.com/u/121693400?v=4" width="100px;" alt=""/><br /><sub><b>Teslenko Anna</b></sub></a><br />
    <td align="center"><a href="https://github.com/isAnastasia"><img src="https://avatars.githubusercontent.com/u/121755328?v=4" width="100px;" alt=""/><br /><sub><b>Gorbunova Anastasia</b></sub></a><br />
    <td align="center"><a href="https://github.com/vano03voin"><img src="https://avatars.githubusercontent.com/u/90224456?v=4" width="100px;" alt=""/><br /><sub><b>Kuznetsov Ivan</b></sub></a><br />

  </tr>
</table>