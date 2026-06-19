# Rules To Create An App

To create an app, **all** you need to do is follow these two rules:
- Create an executable file named `ui.sh` (you can customize the executable file path using CLI arguments).
- The executable file must write the XML UI to a predefined file path stored in the `$XML` variable.
All available features and rules for building your interface can be found in the [tags](tags/window.md) documentation.


---


# Practice: Writing a Counter App


## 1. Initialize the Application

Create a `ui.sh` script that writes XML structure to the `$XML` file.

The application's XML must start with a `<window>` tag.

```sh
#!/bin/sh

cat <<EOF > $XML
<window>
</window>
EOF
```


## 2. Add Elements to the UI

For our counter, we need a `+` button, a `-` button, and a text element to display the current value.

Because `<window>` can only contain one child element, we need a container that can handle multiple children.
Let's use `<row>` to display our elements horizontally.

> [!NOTE]
> `<btn>` can only contain child tags, not raw text.
> 
> To display text inside a button, wrap it in a `<t>` tag like this: `<btn><t>...</t></btn>`

```sh
#!/bin/sh

cat <<EOF > $XML
<window>
    <row>
        <btn><t> - </t></btn>
        <t>0</t>
        <btn><t> + </t></btn>
    </row>
</window>
EOF
```

## 3. Add Reactivity

Right now, our app is not very useful because the buttons do not interact with the displayed number (it remains statically at "0").

### Step A: Make the Value Dynamic

First, we need to represent the "count" using a variable.
Remember that since this is a Shell script, you can dynamically generate your XML however you like.
Here are two ways to achieve this (we will focus on the first approach):

```sh
#!/bin/sh

cat <<EOF > $XML
<window>
    <row>
        <btn><t> - </t></btn>
        <t>$RANDOM</t>
        <btn><t> + </t></btn>
    </row>
</window>
EOF
```

```sh
#!/bin/sh

cat <<EOF > $XML
<window>
    <row>
        <btn><t> - </t></btn>
        <t>{{COUNT}}</t>
        <btn><t> + </t></btn>
    </row>
</window>
EOF

sed -i "s/{{COUNT}}/$RANDOM/" $XML
```

Now, the displayed count is dynamically generated using the built-in `$RANDOM` variable. Each time the app starts, a different number will be shown.

<br><br>

### Step B: Initialize the State

Instead of just showing random numbers, we want to store, edit, and display a persistent counter value.

> [!WARNING]
> You might be **wrongly** tempted to simply declare and initialize a local variable inside the `ui.sh` script like this: :
> 
> ```sh
> #!/bin/sh
> 
> COUNT=0 # <---- Variable initialization
> 
> cat <<EOF > $XML
> <window>
>      <row>
>          <btn><t> - </t></btn>
>          <t>$COUNT</t>
>          <btn><t> + </t></btn>
>      </row>
> </window>
> EOF
> ```
> 
> However, this will not work.
> With every user interaction, the script runs again from the beginning to regenerate the new UI (see the "How it works" section).
> 
> This means `$COUNT` would be reset to `0` every single time the script runs.


To prevent this, you must initialize the variable conditionally, only if it doesn't already exist:

```sh
#!/bin/sh

if test -z "$COUNT"
then
    export COUNT=0
fi

# Shorter alternative: test -z "$COUNT" && export COUNT=0

cat <<EOF > $XML
<window>
    <row>
        <btn><t> - </t></btn>
        <t>$COUNT</t>
        <btn><t> + </t></btn>
    </row>
</window>
EOF
```

This way, the variable is initialized at runtime only when it is not already present in the environment.

> [!tip]
> A good practice is to create an `init_vars.sh` script containing all your variable initializations and simply source it here


<br><br>


### Step C: Manage Interaction (Updating the Variable)

Now, we want to increment and decrement `$COUNT` when the user clicks the respective buttons.

To achieve this, we use the `onclick` attribute of the `<btn>` tag to update the value of `COUNT` globally—for example, using `export COUNT=$((COUNT + 1))`.

```sh
#!/bin/sh

if test -z "$COUNT"
then
    export COUNT=0
fi

cat <<EOF > $XML
<window>
    <row>
        <btn onclick="export COUNT=$((COUNT - 1))">
            <t> - </t>
        </btn>
        <t>$COUNT</t>
        <btn onclick="export COUNT=$((COUNT + 1))">
            <t> + </t>
        </btn>
    </row>
</window>
EOF
```

Remember that this script generates static XML, meaning all `$` variables are evaluated during the Shell execution.
The output XML written to $XML will look like this:

```xml
<window>
    <row>
        <btn onclick="export COUNT=-1">
            <t> - </t>
        </btn>
        <t>0</t>
        <btn onclick="export COUNT=1">
            <t> + </t>
        </btn>
    </row>
</window>
```