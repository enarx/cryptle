document.addEventListener('DOMContentLoaded', () => {
  // Twitter handle
  let twitter = "";

  // Draw empty board
  createSquares();

  // Varibles for Handling Key Presses
  const guessedWords = [[]];
  let availableSpace = 1;
  let guessedWordCount = 0;

  // Modal Event Listener
  const modalHelp = document.querySelector('.modal_help');
  const modalHelpEl = document.querySelector('.close_modal_help');
  modalHelpEl.addEventListener('click', toggleHelpModal);

  const modalStats = document.querySelector('.modal_stats');
  const modalStatsEl = document.querySelector('.close_modal_stats');
  modalStatsEl.addEventListener('click', toggleStatsModal);

  const headerKeys = document.querySelectorAll('header button');

  for (let index = 0; index < headerKeys.length; index++) {
    headerKeys[index].onclick = ({currentTarget}) => {
      const hint = currentTarget.getAttribute('data-key');

      if (hint === 'help'){
        handleHelp();
        return;
      }

      if (hint === 'stats') {
        handleStats();
        return;
      }

    }
  }

  function toggleHelpModal() {
    var currentState = modalHelp.style.display;
  
    // If modal is visible, hide it. Else, display it.
    if (currentState === 'none') {
      modalHelp.style.display = 'block';
    } else {
      modalHelp.style.display = 'none';  
    }
  }

  function toggleStatsModal() {
    var currentState = modalStats.style.display;
  
    // If modal is visible, hide it. Else, display it.
    if (currentState === 'none') {
      modalStats.style.display = 'block';
    } else {
      modalStats.style.display = 'none';  
    }
  }

  function handleHelp() {
    toggleHelpModal();
  }

  function handleStats() {
    toggleStatsModal();

    fetch("./matches", {
      "method": "GET",
    })
    .then(function(response) {
      if (!response.ok) {
        throw Error();
      };
      response.text()
      .then(function(result) {
        let matches = result.trim();

        if (matches == "") {
          matches = "None";
        }

        const statsEl = document.getElementById('cryptle-matches');
        statsEl.textContent = matches;

      })
    }).catch(() => {
      window.alert('Error connecting to Cryptle')
    })

    fetch("./winners", {
      "method": "GET",
    })
    .then(function(response) {
      if (!response.ok) {
        throw Error();
      };
      response.text()
      .then(function(result) {
        let winners = result.trim();

        if (winners == "") {
          winners = "None";
        } else {
          const winnersString = winners.replaceAll(", ",",");
          const winnersArr = winnersString.split(",");
          const winnersMap = winnersArr.reduce((acc, e) => acc.set(e, (acc.get(e) || 0) + 1), new Map());
          const winnersSorted = new Map([...winnersMap.entries()].sort((a, b) => b[1] - a[1]));

          winners = "<ul>";
          for (let [key, value] of winnersSorted) {
            winners += `<li>${key} (${value} points)</li>`;
          }
          winners += "</ul>";
        }

        const statsEl = document.getElementById('cryptle-winners');
        statsEl.innerHTML = winners;
      })
    }).catch(() => {
      window.alert('Error connecting to Cryptle')
    })
  }


  // Keyboard Event Listeners
  const keys = document.querySelectorAll('.keyboard-row button');

  for (let index = 0; index < keys.length; index++) {
    keys[index].onclick = ({target}) => {
      const letter = target.getAttribute('data-key');
  
      if (letter === 'enter'){
        handleSubmitWord();
        return;
      }

      if (letter === 'del') {
        handleDeleteLetter();
        return;
      }

      handleGuessedLetter(letter);
    }
  }

  function handleDeleteLetter() {
    const currentWordArr = getCurrentWordArray();

    if(currentWordArr.length == 0) {
      return;
    }

    const removedLetter = currentWordArr.pop();

    guessedWords[guessedWords.length - 1] = currentWordArr;
    
    const lastLetterEl = document.getElementById(String(availableSpace - 1));

    lastLetterEl.textContent = '';
    availableSpace = availableSpace - 1;
  };


  function getTileColor(color){
    switch(color) {
      case "y":
        return "#ffff99";
      case "g":
        return "#6aaa64";      
      case "b":
        return "#85c0f9";
      case "p":
        return "#cc99cc";
      case "r":
          return "#ff7070";
      default:
          return "#d3d3d3"
    }
  };

  function handleSubmitWord () {
    const currentWordArr = getCurrentWordArray();
    if (currentWordArr.length !== 5){
      window.alert("Not enough letters");
      return;
    }

    const currentWord = currentWordArr.join('');

    var apiCall = "single";
    var multiCheck = document.getElementById("multiCheckbox").checked;

    if (multiCheck) {
      apiCall = "multi";

      if (twitter == "") {
        twitter = window.prompt("Please enter your twitter handle:");

        if (twitter == "") {
          window.alert("Multi-player requires a twitter handle. Switching back to single player.");
          document.getElementById("multiCheckbox").checked = false;
          apiCall = "single";
        }
      }

    }

    fetch(`./${apiCall}?guess=${currentWord}&player=${twitter}`, {
      "method": "GET",
    })
    .then(function(response) {
      if (!response.ok) {
        throw Error();
      };
      response.text()
      .then(function(result) {
        const colors = result.trim();

        const firstLetterId = guessedWordCount * 5 + 1;
        const interval = 200;
  
        currentWordArr.forEach((letter, index) => {
          setTimeout(() => {
            const tileColor = getTileColor(colors[index]);
  
            const letterId = firstLetterId + index;
            const letterEl = document.getElementById(letterId);
            letterEl.classList.add("animate__flipInX");
            letterEl.style = `background-color:${tileColor}`;
          }, interval * index)
        })
  
        guessedWordCount += 1;
  
        guessedWords.push([]);

      })
    }).catch(() => {
      window.alert('Error connecting to Cryptle')
    })
  }  

  function getCurrentWordArray(){
    const numberOfGuessedWords = guessedWords.length;
    return guessedWords[numberOfGuessedWords - 1];
  };

  function handleGuessedLetter(letter){
    const currentWordArray = getCurrentWordArray();
    if (currentWordArray && currentWordArray.length < 5) {

      if (availableSpace == 31) {
        // Reset board
        for (let index = 1; index <= 30; index++) {
          const letterEl = document.getElementById(index);
          letterEl.textContent = "";
          letterEl.classList.remove("animate__flipInX");
          letterEl.style = "background-color:#ffffff";
        }
        availableSpace = 1;
        guessedWordCount = 0;
      }

      currentWordArray.push(letter);

      const availableSpaceEl = document.getElementById(String(availableSpace));
      availableSpace = availableSpace + 1;

      availableSpaceEl.textContent = letter;
    }
  };

  function createSquares() {
    const gameBoard = document.getElementById('board');
    for(let index=0; index<30; index++){
      let square = document.createElement('div');
      square.classList.add('square');
      square.classList.add('animate__animated');
      square.setAttribute('id', index + 1);
      gameBoard.appendChild(square);
    }
  };
})
