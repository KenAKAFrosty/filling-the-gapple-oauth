# Setting up OAuth within Google

Like any OAuth integration, the main questions to answer when setting up with a given provider are:

- How do I get to my developer settings for OAuth
- Where are the list of scopes/how do I unlock scopes for my app?
- Where do I get my Client ID & Secret?
- Where do I set approved callback URLs?

In classic GCP fashion, it's a bit indirect. But it's relatively simple and fast if the steps are clear.

These are those steps.

## 1 ) Head to console menu

Log into cloud console - https://console.cloud.google.com/

(This makes the reasonable assumption you have a google account; if not, you'll have to make one first.)

Open the hamburger menu in the upper left. Click `APIs & Services`

![alt text](<01 - menu.png>)

## 2 ) Go to APIs & Services

You'll land here. Click `OAuth Consent Screen` on the left
![alt text](<02 - apis&services.png>)

## 3 ) OAuth Overview

Hit `Get Started`.

![alt text](<03 - before configured.png>)

## 4 ) Basic info

![alt text](<04 - oauth app config.png>)

## 5 ) Make sure Audience is set to External

![alt text](<05 - audience copy.png>)

## 6 ) Finish up

![alt text](<06 - finish.png>)

## 7 ) Overview with no clients yet

Here you'll hit `Create Oauth client` over on the right
![alt text](<07 - overview create client.png>)

## 8 ) Now you're creating your app client

![alt text](<08 - web app client.png>)

## 9 ) Origins & Redirect URIs

Here you can finally set these.

We probably won't be requesting straight from the browser / using implicit grant flow. So we don't really need to set `Authorized JavaScript origins`, but here's where you would.

Our main focus is registering the redirect URIs. These can be whatever we choose; I've chosen to plan to redirect to /oauth/google , but that's coincidental. Whatever floats your boat.

Just remember to also include the localhost equivalent explicitly for testing.

Hit `Create/Save` at the end

![alt text](<09 - origins & redirect uris.png>)

## 10 ) Created Client, ID available now!

![alt text](<10 - client id.png>)

## 11 ) Hit the `Clients` tab on the left menu

Now you'll see the client you just made. Click on it to see its details again.

![alt text](<11 - clients view.png>)

## 12 ) Client Secret avaialable now!

Grab the secret here, then hit `Data Access` on the left menu
![alt text](<12 - client details, get secret.png>)

## 13 ) Scope time

You won't have any scopes, even the most basic ones, unless you explicitly add them.

Click `Add or remove scopes`
![alt text](<13 - data access.png>)

## 14 ) Select basic scopes

We're assuming that we're using OAuth mostly for an identity provider to keep logins/signups simple, not to engage with a bunch of Google's APIs.

Email/ID/Public Info are chosen here accordingly.

If you did want to do something user-specific with a Google API, here's where you'd find and select the appropriate scope.

![alt text](<14 - add scopes.png>)

## 15 ) Stay Sensitive to Sensitive

If you have any sensitive or restricted scopes, you will likely have to go through an entire app review process. A full submission to Google, requiring their team's manual review, the whole shebang. Not a ride you want to get on unless you really do rely on those functionalities.

Here you can quickly glance and make sure you've only selected `Your non-sensitive scopes`

Save your choices, then head to `Branding` on the left menu real quick.

![alt text](<15 - verify nonsensitive to avoid pain.png>)

## 16 ) Branding - check verification status

Here's a place you can quickly see if verification is required.

![alt text](<16 - check in branding that verification still not required.png>)

## 17 ) Verification Center

You can also go to `Verification Center` on the left to double check verification is not required.

![alt text](<17 - also check verification center still not required.png>)

## 18 ) Important!! Make sure to publish!

Go to `Audience` on the left menu, and hit `Publish app` to be able to actually use it with any arbitrary user.

Without this, it's limited only to test accounts (that's fine at first when you're only testing/staging anyway, just don't forget to publish before attempting to put all this in front of real users!)

![alt text](<18 - audience, publish app.png>)

## 19 ) Congrats!

Now you have your Client ID, Client Secret, approved scopes, and a publish(able) app for public use. You should have all you need to use Google as an OAuth Provider
