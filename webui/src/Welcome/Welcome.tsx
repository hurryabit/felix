import { Title, Text, Anchor } from "@mantine/core";
import * as classes from "./Welcome.css";

import * as wasm from "felix-wasm-bridge";

export function Welcome() {
  const point = wasm.my_point();
  console.log("WebAssemly's point:", point);
  return (
    <>
      <Title className={classes.title} ta="center" mt={100}>
        Welcome to{" "}
        <Text
          inherit
          variant="gradient"
          component="span"
          gradient={{ from: "blue", to: "violet" }}
        >
          felix
        </Text>
      </Title>
      <Text c="dimmed" ta="center" size="lg" maw={580} mx="auto" mt="xl">
        This will become a playground for compiler frontend and programming
        language experiments.
        While felix is still under construction, you might have fun with{" "}
        <Anchor href="https://hurryabit.github.io/rufus">
          rufus
        </Anchor>.
        <br />
        Also, WebAssembly wants to make a point: {JSON.stringify(point)}.
      </Text>
    </>
  );
}
