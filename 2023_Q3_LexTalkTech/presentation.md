---
title: "Things I Hate About Developers"
description: "Talk for LexTalkTech Fall 2023"
author: "Nyefan"
theme: "gaia"
footer: "LexTalk Tech 2023-10-12"
url: "https://nyefan.org/slides/2023-10-12-LexTalk-Tech.html"
---
<!--Good Evening! <pause for response>-->
<!--How are we doing tonight? <pause for response>-->
<!--I only have 15 minutes here, so we're gonna go fast.-->
<!--This presentation and the presenter notes will be available at nyefan.org if you want to see it again-->
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
<!--<eat the mic, lower voice conspiratorially> This conference is quarterly, so that's 53 minutes if you come to all 4-->
<!--Developers, we're gonna talk tonight about a number of action items you can take to make your infrastructure teams' lives a hell of a lot easier and make your software more robust and reliable in the process-->
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
### &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span style="color:red;">6</span>
## <span style="color:red;text-decoration:line-through"><span style="color:#455a64">10</span></span><span style="color:red;font-family:monospace;font-size:0.32em;">‸</span>Things I Hate About You
<!--Well, 6, actually - we don't have time for 10-->

---
### You don't capture and handle `SIGTERM` and `SIGKILL`
###### Kubernetes scale down process*
```rust
remove pod from the network load balancers
send SIGTERM to containers in pod
loop 30 seconds:
  if processes have exited:
    delete resources;
    exit 0;
  sleep;
send SIGKILL to containers in pod
delete resources;
exit 0;
```
###### <span style="font-size:0.5em">*simplified; see the [kubernetes pod lifecycle](https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/) for full details</span>
<!--How many of you work in Kubernetes shops?-->
<!--How many of you have heard of the kubernetes pod lifecycle?-->
<!--<if any hands stay up> good on you - you're the good eggs-->
<!--<else> yeah, that's what I thought - but you're about to learn enough to be dangerous-->
<!--One of the most important functions of kubernetes-->
<!--(aside from providing a standard interface over common deployment structures like networking, resource allocation, workload isolation, process environment, installed libraries...)-->
<!--Ok fine, *an* important function of kubernetes-->
<!--is scaling workloads to meet demand and recovering failed workloads-->
<!--In order to do this without corrupting data or sending customers to 502 pages unnecessarily-->
<!--Services need to respond correctly to the process signals kubernetes uses to manage workloads-->
<!--This is very simple in concept, but it's often forgotten or poorly communicated, so it gets left on the table-->
<!--Just respond to SIGTERM by shedding any stateful load to other instances of the service and shutting down-->
<!--As an added bonus, this will make your rolling deployments much faster, as the kubelet doesn't have to wait 30s to kill each pod-->

---
### You use at most once semantics
###### Example "at least once" network call scheme
```rust
function network_call(data):
  result = HttpClient.verb(data);
  if result.ok():
    return result;
  else if result.error().status_code not in [config.RETRYABLE_ERROR_CODES]:
    log.error(result.error().error_code); 
    return result.error(); // send to dead letter queue
  else:
    return retry(network_call, data)
    
```
###### <span style="font-size:0.5em">*the `network_call` should be idempotent; see [this presentation's repository](https://github.com/Nyefan/Presentations/blob/main/2023_Q3_LexTalkTech/retry.rs) for a more complete example</span>
<!--If you're not using kubernetes, don't worry, I have some work for you, too-->
<!--All too often, I see service crashes or deployment rotations lead to data loss or inconsistency-->
<!--because network calls between services or to external providers are written with naive "at most once" behavior.-->
<!--Just throwing bits out into the ether and hoping they get where they're going-->
<!--(well when you put it that way, it sounds like fun)-->
<!--But it's not very consistent, and consistency is - to my thinking - the most important part of software engineering-->
<!--So every network call should be stuffed into a retry loop of some kind-->
<!--And this is very important...-->
<!--Failed requests should be sent to a dead-letter queue!-->
<!--This is important for detecting incidents: if you don't know requests are failing, you can't respond to the issue-->
<!--It is important for diagnosing incidents: if you don't know *which* requests are failing, you can't fix the issue-->
<!--And it's important for mitigating incidents: you can replay the failed events or reset the event queue to just before failures began-->

---
### Your retries are too fast
###### Example retry scheme
```rust
function retry(network_call, data):
  retry_count = 0
  delay = config.INITIAL_DELAY_MS;
  while retry_count <= config.MAX_RETRIES:
    delay = min(
      config.MAX_DELAY_MS,
      delay * config.BACKOFF_FACTOR + rng.range(0.0..1.0) * config.JITTER_FACTOR_MS
    );
    sleep(delay);
    if (network_call(data).ok()) break;
    
```
###### <span style="font-size:0.5em">*the recursive `network_call` won't work, but slides only have 10 lines</span>
<!--Just don't give me too much of a good thing-->
<!--When you're retrying, make sure to put in some exponential backoff on your requests-->
<!--(The math nerd in me is also compelled to point out that most supposed "exponential backoff" schemes are in fact, geometric rather than exponential, but I digress)-->
<!--This is my default backoff scheme, and I want to point out a few features that mitigate the impact of mass disconnect events-->
<!--external DDOS attacks are child's play to mitigate, but self-inflicted thundering herds are far more difficult-->
<!--First, we wait at least 1-3 milliseconds before trying again the first time - there are times you'll want to wait more or less time, but I find that to be a good default-->
<!--Second, we have a maximum retry delay, usually not more than a second in a backend service, but frontend can be 60 seconds or more-->
<!--Third, I want you all to focus on this JITTER_FACTOR_MS-->
<!--If everyone is disconnected at once, either because of an internal network blip, a session cache invalidation, or anything else-->
<!--They will retry in waves without a jitter factor, and it will take longer to recover-->
<!--By skewing the retry time on a per-client basis, you'll get a more consistent throughput of retries and will be less likely to overwhelm your downstream services-->
<!--And don't worry about what exactly these values should be - your operations engineers will know how to set these levers so long as you provide them.-->

---
### You hard code default config variables
###### Example configuration variable scheme
```rust
ENV = os.environ.get("APPNAME_ENV")

function coalesce(env_var_name, config_var_path):
  return os.environ.get(env_var_name) or 
    config.read(config_var_path, f"${ENV}.yaml") or 
    config.read(config_var_path, "defaults.yaml") or 
    raise Exception()

DOWNSTREAM_SERVICE_URL = coalesce("APPNAME_DOWNSTREAM_SERVICE_URL", "appname.downstream_service.url")
RETRIES_INITIAL_DELAY_MS = coalesce("APPNAME_RETRIES_INITIAL_DELAY_MS", "appname.retries.initial_delay_ms")
RETRIES_MAX_DELAY_MS = coalesce("APPNAME_RETRIES_MAX_DELAY_MS", "appname.retries.max_delay_ms")
RETRIES_MAX_RETRIES = coalesce("APPNAME_RETRIES_MAX_RETRIES", "appname.retries.max_retries")
...

```
###### <span style="font-size:0.5em">*see the [spring boot properties hierarchy](https://docs.spring.io/spring-boot/docs/1.5.6.RELEASE/reference/html/boot-features-external-config.html) for a more complete (and arguably overengineered) list</span>
<!--Now on the subject of levers-->
<!--One of the most common ways I see deployments fail in higher environments or cause incidents in production-->
<!--Is because a new config variable was added into a service with a hardcoded default value-->
<!--which was valid for local development and the dev environment, but then failed in staging or production-->
<!--If you add a config variable, please just raise an Exception if the value couldn't be loaded from the environment-->
<!--and then it will obviously fail in dev if it doesn't get set by the release team, so the bug will never hit customers-->

---
### Your logs don't report the source of errors
###### Rules for good logs <img height="96" src="error.png" style="float:right;margin:230px 10px -250px 0px" title="Error, an error ocurred"/>
```json
{
"Use log levels":
    ["TRACE", "DEBUG", "INFO", "WARN", "ERROR", "FATAL"],
"Prefer structured logging formats":
    ["json", "logfmt", "avro", "protobuf"],
"Avoid multiline logs":
    ["yaml", "formatted stack traces"],
"Include relevant program state": 
    ["ISO8601 time", "userID", "sessionID", "txnID", "flattened stack traces"]
}
```
###### <span style="font-size:0.5em">*This is contentious - see [here](https://softwareengineering.stackexchange.com/questions/312197/benefits-of-structured-logging-vs-basic-logging) and [here](https://gregoryszorc.com/blog/2012/12/06/thoughts-on-logging---part-1---structured-logging/) for discussion</span>
<!--Well, as long as you log the failure, at least-->
<!--I can't tell you how many times I've seen some variation of "Exception: An Error Occurred" in ostensibly production software.-->
<!--Give me a stack trace at least, if not a full (anonymized) stack frame-->
<!--but also, learn how your log aggregation tools work-->
<!--You can save your incident management team a lot of time responding to issues just by making your logs conform with-->
<!--the expected structure of whatever log aggregator you're using-->
<!--The specifics are very organization-dependent, so I just have a few examples of best practices up here-->
<!--but the less time we spend writing yet another service-specific yaml parser, the more time we can spend speeding up your builds-->

---
### You don't speak up
###### Casual bigotry pushes marginalized groups out of the industry
```
Cisgender Women, Queer People, Neurodivergent People, Disabled People, and 
BIPOC are all underrepresented among professional software workers.

Women additionally have a huge exit rate relative to men after 5-10 years.

Casual misogyny, queerphobia, nationalism, white supremacy, and ableism are 
tolerated in this industry, especially from upper management.

A diversity of experience begets a diversity of tactics, which are required 
for delivering great software and for organizing towards a better world.
```
###### <span style="font-size:0.5em">*data pulled from the [stackoverflow developer surveys](https://insights.stackoverflow.com/survey/) and the papers examined in [this 2022 meta-analysis of the literature](https://arxiv.org/pdf/2303.05953.pdf)</span>
<!--<sit down, bring down the energy of the room> Finally, let me tell you a story.-->
<!--I spent some time working for a company that had a compelling mission, a solid engineering team, and an actual product in customers' hands-->
<!--This company was well set up to push through to profitability and acquisition within their funding runway-->
<!--All we had to do was push features required by new customers and keep everything scaling up without too much distress-->
<!--<half laugh> I say "all" like it's that simple - we also had to get stupendously lucky, but that's a given-->
<!--Unfortunately, this company had a serious liability-->
<!--The Director of Engineering was a twat.-->
<!--He would spend an hour every morning on the engineering-department-wide stand-up pontificating about his supposed-->
<!--engineering prowess and berating every engineer as they went through their description of their previous day's work-->
<!--and stamping his feet child.-->
<!--Well, I say *every* engineer, but that's not strictly true - there were some of us who he never yelled at-->
<!--He never yelled at me, for instance.-->
<!--He was, after all, under the impression that I was a neurotypical, perfectly abled, straight, cisgender, white man-->
<!--One out of six... isn't the worst possible score... on a true or false test-->
<!--I was locked in because of a signing bonus I had taken, so I didn't say anything at first-->
<!--After some time, I took it to HR and made it clear that if he ever yelled at *me*, I would quit on the spot regardless of the signing bonus-->
<!--But I never said anything during the standups - I should have, but I didn't-->
<!--As the person with the power to make hiring and firing decisions, he had too much power over me-->
<!--Eventually, a CTO position was created to promote him out of the way, and my manager got his job-->
<!--She was brilliant as both an engineer and a project manager, but the damage had already been done.-->
<!--The employees were too disaffected, and most everyone left as soon as their year was up-->
<!--Most of their staff have been laid off since then, they haven't managed to get another round of seed funding,-->
<!--and their website barely even exists anymore-->
<!--That's not really the happy ending we're looking for is it?-->
<!--So let's make it happy - this is a demonstration of how tolerating bigotry in the workplace can wreck a company-->
<!--but it's also a story of how a diverse team can build something great in the first place-->
<!--and how a diversity of experience that isn't limited to tooling, frameworks, and design patterns allows-->
<!--us to create better software.  My last name-->

---
## Thank You For Your Time
&nbsp;
##### Davis St. Aubin
##### Software Engineer and Consultant
##### consulting@nyefan.org
##### https://www.nyefan.org/categories/#presentations
<!--has a period and a space in it, and you better be damn sure no product I've touched assumes names have any-->
<!--universal rules or that people's genders are immutable or binary-->
<!--By seeking out a diversity of experience, we ensure a corresponding diversity of tactics,-->
<!--allowing us to deliver better software today and organize a better world tomorrow.-->
<!--That! is a much happier ending-->
<!--Thank you for your time.<exit stage right>-->
