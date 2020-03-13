# YetAnotherBar

- **#bar_name** (Window)
  - **#main_box** (Box)
    - (EventBox)
      - **#clock** (Label)
    - (EventBox)
      - **#alsa** (Label)
        - .muted
    - (EventBox)
      - **#mpris** (Label)
        - .playing
    - **#cpu** (Label)
    - **#i3** (Box)
      - (Button) * number of workspaces
        - .focused
        - .urgent
        - :hover
        - :active
      - **#mode** (Label)

<div>
  Window <b>#bar_name</b>
  <div id="main_box">
    Box <b>#main_box</b>
    <main class="flex">
      <div>
        EventBox
        <div>Label <b>#clock</b>
        </div>
      </div>
      <div>
        EventBox
        <div>Label <b>#alsa</b>
          <ul>
            <li>.muted</li>
          </ul>
        </div>
      </div>
      <div>
        EventBox
        <div>Label <b>#mpris</b>
          <ul>
            <li>.playing</li>
          </ul>
        </div>
      </div>
      <div>
        Label <b>#cpu</b>
      </div>
      <div>
        Box <b>#i3</b>     
        <section class="flex">
          <div>Button</div>
          <div>Button</div>
          <div>Button</div>
          <div>Label <b>#mode</b></div>
        </section>
      </div>
    </main>
  </div>
</div>

<style>
div{
  border: 1px solid black;
  padding: 10px;
}
.flex{
  display: flex;
}
#main_box{
  background: rgba(0,0,0,0.2);
}
.flex > div{
  margin-right: 15px;
}
#main_box div{
  background: rgba(0,0,100,0.2);
}
</style>