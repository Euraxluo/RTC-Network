import React, { useEffect, useState } from 'react'
import { Form, Input, Grid, Card, Statistic,Dropdown } from 'semantic-ui-react'

import { useSubstrateState } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

function Main(props) {
  const { api } = useSubstrateState()


  const { keyring } = useSubstrateState()
  const accounts = keyring.getPairs()

  const availableAccounts = []   
  accounts.map(account => {
    return availableAccounts.push({
      key: account.meta.name,
      text: account.meta.name,
      value: account.address,
    })
  })

  
  // The transaction submission status
  const [status, setStatus] = useState('')

  // The currently stored value
  const [currentValue, setCurrentValue] = useState(0)
  const [formValue, setFormValue] = useState(0)

  useEffect(() => {
    let unsubscribe
    api.query.templateModule
      .something(newValue => {
        // The storage value is an Option<u32>
        // So we have to check whether it is None first
        // There is also unwrapOr
        if (newValue.isNone) {
          setCurrentValue('<None>')
        } else {
          setCurrentValue(newValue.unwrap().toNumber())
        }
      })
      .then(unsub => {
        unsubscribe = unsub
      })
      .catch(console.error)

    return () => unsubscribe && unsubscribe()
  }, [api.query.templateModule])


  let channel = null

  const connection = new RTCPeerConnection({ iceServers: [{ urls: 'stun:stun.l.google.com:19302' }] })

  connection.ondatachannel = (event) => {
    console.log('ondatachannel')
    channel = event.channel
    channel.onopen = event => console.log('onopen', event);
    channel.onmessage = (event) => alert(event.data)
  }

  connection.onconnectionstatechange = (event) => (document.getElementById('connectionState').innerText = connection.connectionState) // console.log('onconnectionstatechange', connection.connectionState)
  connection.oniceconnectionstatechange = (event) =>
    (document.getElementById('iceConnectionState').innerText = connection.iceConnectionState) // console.log('oniceconnectionstatechange', connection.iceConnectionState)

  // async function step_1_initiator_create_offer() {
  //   channel = connection.createDataChannel('data')
  //   // channel.onopen = event => console.log('onopen', event)
  //   // channel.onmessage = event => console.log('onmessage', event)
  //   channel.onmessage = (event) => alert(event.data)

  //   connection.onicecandidate = (event) => {
  //     // console.log('onicecandidate', event)
  //     if (!event.candidate) {
  //       document.getElementById('createdOffer').value = JSON.stringify(connection.localDescription)
  //       document.getElementById('createdOffer').hidden = false
  //     }
  //   }

  //   const offer = await connection.createOffer()
  //   await connection.setLocalDescription(offer)
  // }

  // async function step_2_accept_remote_offer() {
  //   const offer = JSON.parse(document.getElementById('remoteOffer').value)
  //   await connection.setRemoteDescription(offer)
  // }

  // async function step_3_create_answer() {
  //   connection.onicecandidate = (event) => {
  //     // console.log('onicecandidate', event)
  //     if (!event.candidate) {
  //       document.getElementById('createdAnswer').value = JSON.stringify(connection.localDescription)
  //       document.getElementById('createdAnswer').hidden = false
  //     }
  //   }

  //   const answer = await connection.createAnswer()
  //   await connection.setLocalDescription(answer)
  // }

  // async function step_4_accept_answer() {
  //   const answer = JSON.parse(document.getElementById('remoteAnswer').value)
  //   await connection.setRemoteDescription(answer)
  // }

  // async function send_text() {
  //   const text = document.getElementById('text').value

  //   channel.send(text)
  // }


  return (
    <Grid.Column width={8}>
      <h1>RTC Module</h1>
      <Card>
        {/* <table width="100%" border="1">
          <tr>
            <th>#</th>
            <th>initiator</th>
            <th>peer</th>
          </tr>
          <tr>
            <td>step 1</td>
            <td>
              <input type="button" value="create offer" onclick="step_1_initiator_create_offer()" />
              <input id="createdOffer" type="text" hidden />
            </td>
            <td></td>
          </tr>
          <tr>
            <td>step 2</td>
            <td></td>
            <td>
              <input id="remoteOffer" type="text" placeholder="offer from initiator" />
              <input type="button" value="accept offer" onclick="step_2_accept_remote_offer()" />
            </td>
          </tr>
          <tr>
            <td>step 3</td>
            <td></td>
            <td>
              <input type="button" value="create answer" onclick="step_3_create_answer()" />
              <input id="createdAnswer" type="text" hidden />
            </td>
          </tr>
          <tr>
            <td>step 4</td>
            <td>
              <input id="remoteAnswer" type="text" placeholder="answer from peer" />
              <input type="button" value="accept answer" onclick="step_4_accept_answer()" />
            </td>
            <td></td>
          </tr>
        </table>
        <hr />
        <input id="text" type="text" />
        <input type="button" value="send" onclick="send_text()" />
        <hr />

        <table border="1">
          <tr>
            <th colspan="2">connection</th>
          </tr>
          <tr>
            <th>connectionState</th>
            <td id="connectionState">unknown</td>
          </tr>
          <tr>
            <th>iceConnectionState</th>
            <td id="iceConnectionState">unknown</td>
          </tr>
        </table> */}
      </Card>

      <Card centered>
        <Card.Content textAlign="center">
          <Statistic label="RTC Recive Value" value={currentValue} />
        </Card.Content>
      </Card>

      <Form.Field>
          <Dropdown
            placeholder="Select from available addresses"
            fluid
            selection
            search
            options={availableAccounts}
            state="addressTo"
            // onChange={onChange}
          />
        </Form.Field>

      <Form>
        <Form.Field>
          <Input
            label="RTC Send Value"
            state="newValue"
            type="string"
            onChange={(_, { value }) => setFormValue(value)}
          />
        </Form.Field>
        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            label="Send to Another Messsage"
            type="SIGNED-TX"
            setStatus={setStatus}
            attrs={{
              palletRpc: 'templateModule',
              callable: 'doSomething',
              inputParams: [formValue],
              paramFields: [true],
            }}
          />
        </Form.Field>
        <div style={{ overflowWrap: 'break-word' }}>{status}</div>
      </Form>
    </Grid.Column>
  )
}

export default function TemplateModule(props) {
  const { api } = useSubstrateState()
  return api.query.templateModule && api.query.templateModule.something ? (
    <Main {...props} />
  ) : null
}
