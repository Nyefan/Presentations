---
title: "11 Things I Hate About You"
description: "Talk for LexTalkTech Fall 2023"
author: "Nyefan"
keywords: "devops,kubernetes"
theme: "gaia"
footer: "LexTalk Tech 2023-09"
url: "https://nyefan.org/2023_Q3_LexTalkTech"
image: "https://nyefan.org/2023_Q3_LexTalkTech/icon.webp"
---
<!--Good Evening! <pause for response>-->
<!--How are we doing tonight? <pause for response>-->
<!--I only have 15 minutes here, so we're gonna go fast.-->
<!--This presentation and the presenter notes will be available at presentations.nyefan.org if you want to see it again-->
<!--The code for this presentation is available at github.com/Nyefan/Presentations-->

---
## <br />
# DEVELOPERS
### <sub><br /></sub>
<!--Developers! <short pause>-->
<!--I'm gonna make you think about operations tonight.-->
<!--Your devops engineers are gonna love me.-->
<!--<hand to ear> Sorry, it's Site Reliability now?-->
<!--<smiling, slightly slower, and with satisfaction> Platform Engineering-->
<!--How many process and operations engineers of different flavors do we have tonight, raise your hands?-->
<!--You guys already know this, you can go to sleep for the next 13 minutes and 15 seconds-->
<!--<eat the mic, lower voice conspiratorially> This talk is quarterly, so that's 53 minutes if you come to all 4-->
<!--Developers, we're gonna talk tonight about a number of action items you can take to make your platform teams' lives a hell of a lot easier and make your software more robust and reliable in the process-->
<!--Without further ado, here is...-->

---
## <br />
# DEVELOPERS
### <br />
## 10 Things I Hate About You
<!--10 Things I Hate About You.-->

---
## <br />
# DEVELOPERS
### &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span style="color:red;">7</span>
## <span style="color:red;text-decoration:line-through"><span style="color:#455a64">10</span></span><span style="color:red;font-family:monospace;font-size:0.62em;">‸</span>Things I Hate About You
<!--Well, 7, actually - we don't have time for 10-->

---
### You don't capture and handle `SIGTERM` and `SIGKILL`
###### Kubernetes scale down process*
```
remove pod from the network load balancers
send SIGTERM to containers running in pod
loop 30 seconds:
  if processes have exited:
    delete resources;
    exit 0;
  sleep;
send SIGKILL to containers running in pod
delete resources;
exit 0;
```
###### <span style="font-size:0.5em;right:30px;text-align:right">*simplified; see the [kubernetes pod lifecycle](https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/) for full details</span>
---
### You use at most once semantics
###### Preferred network call scheme
```
function network_call(data):
  result = HttpClient.verb(data);
  if result.Ok():
    return result;
  else:
    return retry(network_call, data)
```
- the `network_call` should be idempotent
- 
---
### Your retries are too fast
###### Preferred retry scheme
```
function retry(network_call, data):
  
```
### 