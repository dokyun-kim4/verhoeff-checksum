import React from "react";
import { useEffect, useState } from "react";
import init, { validate_checksum } from "./pkg/rust_code";

const Validate = () => {
  const [inputText, setInputText] = useState("");
  const [inputField, setInputField] = useState("inputField");

  async function checkValid(input) {
    await init();
    if(validate_checksum(input) == true) {
      setInputField("inputField valid");
    } else {
      setInputField("inputField invalid");
    }
  }

  const handleTextInput = (e) => {
    const inputValue = e.target.value;
    if (/^\d*$/.test(inputValue)) {
    setInputText(inputValue);
    console.log("inputText:", inputText);
  }
  };

  useEffect(() => {
    if (inputText.length > 0) {
      checkValid(inputText)
    } else {
      setInputField("inputField")
    }
  }, [inputText]);

  return (
    <div className="validate">
      <div className="title">Validate checksum</div>
      <input
          className={inputField}
          type="text"
          value={inputText}
          onChange={handleTextInput}
          placeholder="Input number here"
        />
    </div>
  );
};

export default Validate;
