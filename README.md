# Bonsai app

For junction 2022 Reaktor challenge.

[Figma designs](https://www.figma.com/file/ny3EfB1YcIUWtRSUBPG1TF/Mental-Health-Bonsai?node-id=0%3A1)

## Introduction

Very often mental health problems arise because of a lack of certain important things in your life. When individuals are down on their luck, it might be difficult to pinpoint the exact root of the problem, or get an idea how to solve the problem. This might make it hard for these individuals to perform even seemingly basic tasks. The bonsai™️ helps show its users a path to improve their situation by little steps.

At the center of this app is a bonsai tree. Users start with a little sapling, which needs to be grown into a tree over time by growing out branches from the base. Branches can be grown by completing simple tasks. The more these tasks you complete, the bigger and more beautiful your tree will grow. The user can customize and build the tree to look like they want by choosing the growth direction and length of the branch as well as where to place leaves. The user would be able to complete only a handful of tasks every day, so that they don’t grow the tree too quickly and miss out on the routine building aspect of the app.

There are many different kinds of tasks to choose from. The app will prioritize the tasks according to the user's needs. When the app is started for the first time, a questionnaire is presented to the user, which tries to map with questions, which areas of life the user needs most improvement with. When growing your bonsai, the tasks will progress to be “more difficult” little by little. If for example you have social anxiety and you want to socialize, the first task is not  going to be singing karaoke at a local bar to a crowd of people. At the start the threshold is much smaller, for example “comment on one social media post”. This way, users won’t get discouraged at the beginning and progress to their goals with smaller, more comfortable steps.

## Tech

We will build our app front-end with svelte-kit and the backend with Rust and Rocket web framework. For deployment we used Vercel for the frontend and Shuttle for the backend.

In the future, the app should be made to be used with mobile devices. With svelte, this could be achieved with Rust-based tauri, when it will get mobile support. Otherwise React-Native or similar will suffice.

The app requires only a username and a password, and no information that can be used to identify users. The tree structure is saved in the database.

## Missing features

- The tasks that the user is given aren’t personalized based on their needs due to time constraints. They come from a database and are chosen randomly.
Some kind of limit not to let users grow the bonsai too many times in one day. It’s currently too easy to just speedrun through, which isn’t the point of the app.

- Make the bonsai also need regular watering / trimming / other activities that enhance the experience and motivate the user

- Some kind of social aspect, where you could interact anonymously with other people's bonsai trees by watering them, etc.

- And there is no more space to write... 
