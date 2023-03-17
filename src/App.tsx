import { useState, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { getCurrent, LogicalSize, appWindow } from "@tauri-apps/api/window";
import {
  Card,
  CardBody,
  Flex,
  Box,
  Heading,
  IconButton,
  Button,
  OrderedList,
  ListItem,
  InputGroup,
  InputRightElement,
  Input,
} from "@chakra-ui/react";
import { BiSearch } from "react-icons/bi";
import { BsClipboardPlus } from "react-icons/bs";
import { MdExpandMore } from "react-icons/md";
import { AiOutlineSound } from "react-icons/ai";
import { j } from "@tauri-apps/api/event-2a9960e7";
// import "./App.css";

type Definition = {
  meta: {
    ["app-shortdef"]: {
      hw: string;
      fl: string;
      def: string[];
    };
  };
  hwi: {
    hw: string;
    prs: {
      ipa: string;
      sound: {
        audio: string;
      };
    }[];
  };
};

const parseDef = (defStr: string): Definition => {
  let def = JSON.parse(defStr);
  if (Array.isArray(def)) {
    def = def[0];
  }
  // console.log(def);
  return def;
};

let a = 0;

function App() {
  const cardRef = useRef<HTMLInputElement | null>(null);
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [def, setDef] = useState<Definition | null>(null);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  useEffect(() => {
    if (cardRef && cardRef.current) {
      let width = cardRef.current.offsetWidth;
      let height = cardRef.current.offsetHeight;
      console.log(width, height);
      getCurrent().setSize(new LogicalSize(width, height));
    }
  }, [def]);

  useEffect(() => {
    const unlisten = appWindow.listen<string>("showDef", (event) => {
      a = a + 1;
      if (event.payload) {
        let new_def = parseDef(event.payload);
        if (new_def.meta) {
          const hw = new_def.meta["app-shortdef"].hw.split(/:/)[0];
          new_def.meta["app-shortdef"].hw = hw;
          setDef(new_def);
        }
      }
    });
  }, []);

  return (
    <Card maxW="md" ref={cardRef}>
      {/* <CardHeader></CardHeader> */}
      <CardBody>
        <Flex>
          <Flex flex="1" gap="4" alignItems="center" flexWrap="wrap">
            <Box>
              <InputGroup size="sm">
                <Input
                  placeholder={def?.meta && def.meta["app-shortdef"]?.hw}
                />
                <InputRightElement width="2.5rem">
                  <IconButton
                    h="1.75rem"
                    size="sm"
                    onClick={() => {
                      console.log("search");
                    }}
                    variant="ghost"
                    colorScheme="gray"
                    aria-label="See menu"
                    icon={<BiSearch />}
                  />
                </InputRightElement>
              </InputGroup>
              <Button flex="1" variant="ghost" leftIcon={<AiOutlineSound />}>
                {def?.hwi?.prs[0]?.ipa}
              </Button>
            </Box>
          </Flex>
          <IconButton
            variant="ghost"
            colorScheme="gray"
            aria-label="See menu"
            onClick={() => {
              console.log("clicked");
            }}
            icon={<BsClipboardPlus />}
          />
        </Flex>
        <Heading size="xs">{def?.meta && def.meta["app-shortdef"]?.fl}</Heading>
        <OrderedList>
          {def?.meta &&
            def?.meta["app-shortdef"]?.def?.map((d, idx) => (
              <ListItem key={idx}>{d}</ListItem>
            ))}
        </OrderedList>
      </CardBody>
      {/* <Image
        objectFit="cover"
        src="https://images.unsplash.com/photo-1531403009284-440f080d1e12?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1770&q=80"
        alt="Chakra UI"
      /> */}

      {/* <CardFooter
        justify="space-between"
        flexWrap="wrap"
        sx={{
          "& > button": {
            minW: "136px",
          },
        }}
      >
          <IconButton
            variant="ghost"
            colorScheme="gray"
            aria-label="See menu"
            onClick={() => {
              console.log("clicked");
            }}
            icon={<MdExpandMore />}
          />
      </CardFooter> */}
    </Card>
  );
}

export default App;
