import { useState, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { getCurrent, LogicalSize, appWindow } from "@tauri-apps/api/window";
import { open } from '@tauri-apps/api/shell';
import {
  Card,
  CardBody,
  Flex,
  Spacer,
  Heading,
  IconButton,
  Button,
  OrderedList,
  Tooltip,
  ListItem,
  InputGroup,
  InputRightElement,
  Input,
} from "@chakra-ui/react";
import { BiSearch } from "react-icons/bi";
// import { BsClipboardPlus } from "react-icons/bs";
import { AiOutlineSound } from "react-icons/ai";
import { MdOpenInBrowser } from "react-icons/md";
// import "./App.css";

type Definition = {
  hw: string,
  fl: string,
  def: string[],
  prs: {
    ipa: string
    audio: string,
  }[]
};

function parseEntry(text: string = ''): any {
  if (Array.isArray(text))
    return parseEntry(text[0]?.[0]?.[1] || '');

  return text
    .replace(/^\{bc\}/, '')
    .replace(/( )?\{bc\}/g, ': ')
    .replace(/( )?\{dx\}/g, '<br /><small>')
    .replace(/( )?\{\/dx\}/g, '</small>')
    .replace(/(?:\{(?:sx|dxt|a_link|d_link|et_link|i_link|mat)\|)([\w\s.,:+-]+)(?:[|])?([\w\s.,:+-]+)?(?:\|)?(?:\d+)?\}/g, (_, text, href) =>
      `<a href="?q=${href || text}">${text}</a>`
    )
    .replace(/(?:\{it\})([\w\s.,:+-]+)(?:\{\/it\})/g, (_, text) => `<em>${text}</em>`)
    .replace(/(?:\{)(\/)?(inf|sup)(\})/g, (_, slash, tag) =>
      `<${slash || ''}${tag === 'sup' ? 'sup' : 'sub'}>`
    );
}

// function parseCaption(text: string = '') {
//   return text
//     .replace(/(?:\{it\})([\w\s.,:+-]+)(?:\{\/it\} )/g, (_, text) => `<br /><em>${text}</em> `)
// }

const parseDef = (defStr: string): Definition => {
  let def = JSON.parse(defStr);
  if (Array.isArray(def)) {
    def = def[0];
  }
  return def;
};

let a = 0;

function App() {
  const cardRef = useRef<HTMLInputElement | null>(null);
  const [def, setDef] = useState<Definition | null>(null);
  const parseAndSetDef = (payload: string) => {
    let new_def = parseDef(payload);
    setDef(new_def);
  }
  const [word, setWord] = useState<string>("");

  function handleInputChange(e: any) {
    setWord(e.target.value);
  }
  async function lookup() {
    const res = await invoke("lookup", { word });
    console.log(res);
    parseAndSetDef(res as string)
  }

  useEffect(() => {
    if (cardRef && cardRef.current) {
      let width = cardRef.current.offsetWidth;
      let height = cardRef.current.offsetHeight;
      getCurrent().setSize(new LogicalSize(width, height));
    }
    const hw = def?.hw;
    if (hw) {
      setWord(hw)
    }
  }, [def]);

  useEffect(() => {
    const unlisten = appWindow.listen<string>("showDef", (event) => {
      a = a + 1;
      if (event.payload) {
        parseAndSetDef(event.payload)
      }
    });
  }, []);

  return (
    <Card maxW="md" ref={cardRef}>
      <CardBody>
        <Flex flex="1" gap="4" alignItems="center">
          <InputGroup size="sm">
            <Input
              placeholder={word}
              onChange={handleInputChange}
              type="search"
              value={word}
            />
            <InputRightElement width="2.5rem">
              <IconButton
                h="1.75rem"
                size="sm"
                onClick={() => {
                  lookup();
                }}
                variant="ghost"
                colorScheme="gray"
                aria-label="See menu"
                icon={<BiSearch />}
              />
            </InputRightElement>
          </InputGroup>
          <Spacer />
          <Flex>
            {def ?
              <Tooltip label="open detail in browser">
                <IconButton
                  variant="ghost"
                  colorScheme="gray"
                  aria-label="See menu"
                  onClick={() => {
                    open(`https://www.merriam-webster.com/dictionary/${word}`)
                  }}
                  icon={<MdOpenInBrowser />}
                />
              </Tooltip> : null
            }
            {/*
              <Tooltip label="add to note">
                <IconButton
                  variant="ghost"
                  colorScheme="gray"
                  aria-label="See menu"
                  onClick={() => {
                    console.log("add to note");
                  }}
                  icon={<BsClipboardPlus />}
                />
              </Tooltip>
              */}
          </Flex>
        </Flex>
        <Button borderRadius="full" flex="1" variant="ghost" leftIcon={<AiOutlineSound />} onClick={() => {
          const mp3 = def?.prs[0]?.audio;
          if (mp3) {
            let subDir = mp3[0];
            if (mp3.startsWith("bix")) {
              subDir = "bix"
            } else if (mp3.startsWith("gg")) {
              subDir = "gg"
            } else if (mp3.startsWith("_")) {
              subDir = "number"
            }
            const format = "mp3";
            const mp3Url = `https://media.merriam-webster.com/audio/prons/en/us/${format}/${subDir}/${mp3}.${format}`
            console.log(mp3Url);
            new Audio(mp3Url).play();
          }
        }}>
          {def?.prs[0]?.ipa}
        </Button>
        {/* <Heading size="xs">{def?.meta && def.meta["app-shortdef"]?.fl}</Heading> */}
        <OrderedList>
          {def?.def.map((d, idx) => (
              <ListItem key={idx} dangerouslySetInnerHTML={{ __html: parseEntry(d) }}></ListItem>
            ))}
        </OrderedList>
      </CardBody>
    </Card>
  );
}

export default App;
