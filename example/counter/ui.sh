#!/bin/sh

test -z "$COUNT" && export COUNT=0

cat <<EOF > $XML
<window
    width='100'
    height='$((COUNT * 10 + 50))'
>
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