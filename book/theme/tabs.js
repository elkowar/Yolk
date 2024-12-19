function openTab(evt, group, tab) {
  const tabcontent = document.getElementsByClassName("tabcontent");

  for (let i = 0; i < tabcontent.length; i++) {
    if (tabcontent[i].getAttribute("group") === group) {
      tabcontent[i].style.display = "none";
    }
  }

  const tablinks = document.getElementsByClassName("tablinks");

  for (let i = 0; i < tablinks.length; i++) {
    if (tabcontent[i].getAttribute("group") === group) {
      tablinks[i].className = tablinks[i].className.replace(" active", "");
    }
  }

  document.getElementById(`${group}-${tab}`).style.display = "block";
  evt.currentTarget.className += " active";
}
