<h1>Midi Status Bytes List</h1>
<p>from <a href="https://midi.org/expanded-midi-1-0-messages-list">https://midi.org/expanded-midi-1-0-messages-list</a></p>
<p>
The following table lists MIDI 1.0 Status Bytes in binary numerical order (adapted from “MIDI by the Numbers” by D. Valenti, Electronic Musician 2/88, and updated by the MIDI Manufacturers Association.) This table is intended as an overview of MIDI, and is by no means complete.
</p>
<aside>
WARNING! Details about implementing these messages can dramatically impact compatibility with other products. We strongly recommend consulting the official MMA Detailed MIDI Specification for additional information.
</aside>
<table border="1" align="center">
<tbody>
<tr align="center" valign="top">
<td colspan="4"><strong>Table 2: Expanded Status Bytes List</strong></td>
</tr>
<tr align="center" valign="top">
<td colspan="2">STATUS BYTE</td>
<td colspan="2">DATA BYTES</td>
</tr>
<tr align="center" valign="top">
<td>1st Byte Value
<p>Binary |Hex| Dec</p>
</td>
<td>Function</td>
<td>2nd Byte</td>
<td>3rd Byte</td>
</tr>
<tr valign="top">
<td>10000000= 80= 128</td>
<td>Chan 1 Note off</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10000001= 81= 129</td>
<td>Chan 2 Note off</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10000010= 82= 130</td>
<td>Chan 3 Note off</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10000011= 83= 131</td>
<td>Chan 4 Note off</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10000100= 84= 132</td>
<td>Chan 5 Note off</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10000101= 85= 133</td>
<td>Chan 6 Note off</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10000110= 86= 134</td>
<td>Chan 7 Note off</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10000111= 87= 135</td>
<td>Chan 8 Note off</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10001000= 88= 136</td>
<td>Chan 9 Note off</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10001001= 89= 137</td>
<td>Chan 10 Note off</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10001010= 8A= 138</td>
<td>Chan 11 Note off</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10001011= 8B= 139</td>
<td>Chan 12 Note off</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10001100= 8C= 140</td>
<td>Chan 13 Note off</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10001101= 8D= 141</td>
<td>Chan 14 Note off</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10001110= 8E= 142</td>
<td>Chan 15 Note off</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10001111= 8F= 143</td>
<td>Chan 16 Note off</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10010000= 90= 144</td>
<td>Chan 1 Note on</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10010001= 91= 145</td>
<td>Chan 2 Note on</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10010010= 92= 146</td>
<td>Chan 3 Note on</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10010011= 93= 147</td>
<td>Chan 4 Note on</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10010100= 94= 148</td>
<td>Chan 5 Note on</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10010101= 95= 149</td>
<td>Chan 6 Note on</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10010110= 96= 150</td>
<td>Chan 7 Note on</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10010111= 97= 151</td>
<td>Chan 8 Note on</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10011000= 98= 152</td>
<td>Chan 9 Note on</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10011001= 99= 153</td>
<td>Chan 10 Note on</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10011010= 9A= 154</td>
<td>Chan 11 Note on</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10011011= 9B= 155</td>
<td>Chan 12 Note on</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10011100= 9C= 156</td>
<td>Chan 13 Note on</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10011101= 9D= 157</td>
<td>Chan 14 Note on</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10011110= 9E= 158</td>
<td>Chan 15 Note on</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10011111= 9F= 159</td>
<td>Chan 16 Note on</td>
<td>Note Number (0-127)</td>
<td>Note Velocity (0-127)</td>
</tr>
<tr valign="top">
<td>10100000= A0= 160</td>
<td>Chan 1 Polyphonic Aftertouch</td>
<td>Note Number (0-127)</td>
<td>Pressure (0-127)</td>
</tr>
<tr valign="top">
<td>10100001= A1= 161</td>
<td>Chan 2 Polyphonic Aftertouch</td>
<td>Note Number (0-127</td>
<td>Pressure (0-127)</td>
</tr>
<tr valign="top">
<td>10100010= A2= 162</td>
<td>Chan 3 Polyphonic Aftertouch</td>
<td>Note Number (0-127</td>
<td>Pressure (0-127)</td>
</tr>
<tr valign="top">
<td>10100011= A3= 163</td>
<td>Chan 4 Polyphonic Aftertouch</td>
<td>Note Number (0-127</td>
<td>Pressure (0-127)</td>
</tr>
<tr valign="top">
<td>10100100= A4= 164</td>
<td>Chan 5 Polyphonic Aftertouch</td>
<td>Note Number (0-127</td>
<td>Pressure (0-127)</td>
</tr>
<tr valign="top">
<td>10100101= A5= 165</td>
<td>Chan 6 Polyphonic Aftertouch</td>
<td>Note Number (0-127</td>
<td>Pressure (0-127)</td>
</tr>
<tr valign="top">
<td>10100110= A6= 166</td>
<td>Chan 7 Polyphonic Aftertouch</td>
<td>Note Number (0-127</td>
<td>Pressure (0-127)</td>
</tr>
<tr valign="top">
<td>10100111= A7= 167</td>
<td>Chan 8 Polyphonic Aftertouch</td>
<td>Note Number (0-127</td>
<td>Pressure (0-127)</td>
</tr>
<tr valign="top">
<td>10101000= A8= 168</td>
<td>Chan 9 Polyphonic Aftertouch</td>
<td>Note Number (0-127</td>
<td>Pressure (0-127)</td>
</tr>
<tr valign="top">
<td>10101001= A9= 169</td>
<td>Chan 10 Polyphonic Aftertouch</td>
<td>Note Number (0-127</td>
<td>Pressure (0-127)</td>
</tr>
<tr valign="top">
<td>10101010= AA= 170</td>
<td>Chan 11 Polyphonic Aftertouch</td>
<td>Note Number (0-127</td>
<td>Pressure (0-127)</td>
</tr>
<tr valign="top">
<td>10101011= AB= 171</td>
<td>Chan 12 Polyphonic Aftertouch</td>
<td>Note Number (0-127</td>
<td>Pressure (0-127)</td>
</tr>
<tr valign="top">
<td>10101100= AC= 172</td>
<td>Chan 13 Polyphonic Aftertouch</td>
<td>Note Number (0-127</td>
<td>Pressure (0-127)</td>
</tr>
<tr valign="top">
<td>10101101= AD= 173</td>
<td>Chan 14 Polyphonic Aftertouch</td>
<td>Note Number (0-127</td>
<td>Pressure (0-127)</td>
</tr>
<tr valign="top">
<td>10101110= AE= 174</td>
<td>Chan 15 Polyphonic Aftertouch</td>
<td>Note Number (0-127</td>
<td>Pressure (0-127)</td>
</tr>
<tr valign="top">
<td>10101111= AF= 175</td>
<td>Chan 16 Polyphonic Aftertouch</td>
<td>Note Number (0-127</td>
<td>Pressure (0-127)</td>
</tr>
<tr valign="top">
<td>10110000= B0= 176</td>
<td>Chan 1 Control/Mode Change</td>
<td>see Table 3</td>
<td>see Table 3</td>
</tr>
<tr valign="top">
<td>10110001= B1= 177</td>
<td>Chan 2 Control/Mode Change</td>
<td>see Table 3</td>
<td>see Table 3</td>
</tr>
<tr valign="top">
<td>10110010= B2= 178</td>
<td>Chan 3 Control/Mode Change</td>
<td>see Table 3</td>
<td>see Table 3</td>
</tr>
<tr valign="top">
<td>10110011= B3= 179</td>
<td>Chan 4 Control/Mode Change</td>
<td>see Table 3</td>
<td>see Table 3</td>
</tr>
<tr valign="top">
<td>10110100= B4= 180</td>
<td>Chan 5 Control/Mode Change</td>
<td>see Table 3</td>
<td>see Table 3</td>
</tr>
<tr valign="top">
<td>10110101= B5= 181</td>
<td>Chan 6 Control/Mode Change</td>
<td>see Table 3</td>
<td>see Table 3</td>
</tr>
<tr valign="top">
<td>10110110= B6= 182</td>
<td>Chan 7 Control/Mode Change</td>
<td>see Table 3</td>
<td>see Table 3</td>
</tr>
<tr valign="top">
<td>10110111= B7= 183</td>
<td>Chan 8 Control/Mode Change</td>
<td>see Table 3</td>
<td>see Table 3</td>
</tr>
<tr valign="top">
<td>10111000= B8= 184</td>
<td>Chan 9 Control/Mode Change</td>
<td>see Table 3</td>
<td>see Table 3</td>
</tr>
<tr valign="top">
<td>10111001= B9= 185</td>
<td>Chan 10 Control/Mode Change</td>
<td>see Table 3</td>
<td>see Table 3</td>
</tr>
<tr valign="top">
<td>10111010= BA= 186</td>
<td>Chan 11 Control/Mode Change</td>
<td>see Table 3</td>
<td>see Table 3</td>
</tr>
<tr valign="top">
<td>10111011= BB= 187</td>
<td>Chan 12 Control/Mode Change</td>
<td>see Table 3</td>
<td>see Table 3</td>
</tr>
<tr valign="top">
<td>10111100= BC= 188</td>
<td>Chan 13 Control/Mode Change</td>
<td>see Table 3</td>
<td>see Table 3</td>
</tr>
<tr valign="top">
<td>10111101= BD= 189</td>
<td>Chan 14 Control/Mode Change</td>
<td>see Table 3</td>
<td>see Table 3</td>
</tr>
<tr valign="top">
<td>10111110= BE= 190</td>
<td>Chan 15 Control/Mode Change</td>
<td>see Table 3</td>
<td>see Table 3</td>
</tr>
<tr valign="top">
<td>10111111= BF= 191</td>
<td>Chan 16 Control/Mode Change</td>
<td>see Table 3</td>
<td>see Table 3</td>
</tr>
<tr valign="top">
<td>11000000= C0= 192</td>
<td>Chan 1 Program Change</td>
<td>Program # (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11000001= C1= 193</td>
<td>Chan 2 Program Change</td>
<td>Program # (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11000010= C2= 194</td>
<td>Chan 3 Program Change</td>
<td>Program # (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11000011= C3= 195</td>
<td>Chan 4 Program Change</td>
<td>Program # (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11000100= C4= 196</td>
<td>Chan 5 Program Change</td>
<td>Program # (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11000101= C5= 197</td>
<td>Chan 6 Program Change</td>
<td>Program # (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11000110= C6= 198</td>
<td>Chan 7 Program Change</td>
<td>Program # (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11000111= C7= 199</td>
<td>Chan 8 Program Change</td>
<td>Program # (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11001000= C8= 200</td>
<td>Chan 9 Program Change</td>
<td>Program # (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11001001= C9= 201</td>
<td>Chan 10 Program Change</td>
<td>Program # (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11001010= CA= 202</td>
<td>Chan 11 Program Change</td>
<td>Program # (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11001011= CB= 203</td>
<td>Chan 12 Program Change</td>
<td>Program # (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11001100= CC= 204</td>
<td>Chan 13 Program Change</td>
<td>Program # (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11001101= CD= 205</td>
<td>Chan 14 Program Change</td>
<td>Program # (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11001110= CE= 206</td>
<td>Chan 15 Program Change</td>
<td>Program # (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11001111= CF= 207</td>
<td>Chan 16 Program Change</td>
<td>Program # (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11010000= D0= 208</td>
<td>Chan 1 Channel Aftertouch</td>
<td>Pressure (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11010001= D1= 209</td>
<td>Chan 2 Channel Aftertouch</td>
<td>Pressure (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11010010= D2= 210</td>
<td>Chan 3 Channel Aftertouch</td>
<td>Pressure (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11010011= D3= 211</td>
<td>Chan 4 Channel Aftertouch</td>
<td>Pressure (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11010100= D4= 212</td>
<td>Chan 5 Channel Aftertouch</td>
<td>Pressure (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11010101= D5= 213</td>
<td>Chan 6 Channel Aftertouch</td>
<td>Pressure (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11010110= D6= 214</td>
<td>Chan 7 Channel Aftertouch</td>
<td>Pressure (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11010111= D7= 215</td>
<td>Chan 8 Channel Aftertouch</td>
<td>Pressure (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11011000= D8= 216</td>
<td>Chan 9 Channel Aftertouch</td>
<td>Pressure (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11011001= D9= 217</td>
<td>Chan 10 Channel Aftertouch</td>
<td>Pressure (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11011010= DA= 218</td>
<td>Chan 11 Channel Aftertouch</td>
<td>Pressure (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11011011= DB= 219</td>
<td>Chan 12 Channel Aftertouch</td>
<td>Pressure (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11011100= DC= 220</td>
<td>Chan 13 Channel Aftertouch</td>
<td>Pressure (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11011101= DD= 221</td>
<td>Chan 14 Channel Aftertouch</td>
<td>Pressure (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11011110= DE= 222</td>
<td>Chan 15 Channel Aftertouch</td>
<td>Pressure (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11011111= DF= 223</td>
<td>Chan 16 Channel Aftertouch</td>
<td>Pressure (0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11100000= E0= 224</td>
<td>Chan 1 Pitch Bend Change</td>
<td>Pitch Bender LSB (0-127)</td>
<td>Pitch Bender MSB (0-127)</td>
</tr>
<tr valign="top">
<td>11100001= E1= 225</td>
<td>Chan 2 Pitch Bend Change</td>
<td>Pitch Bender LSB (0-127)</td>
<td>Pitch Bender MSB (0-127)</td>
</tr>
<tr valign="top">
<td>11100010= E2= 226</td>
<td>Chan 3 Pitch Bend Change</td>
<td>Pitch Bender LSB (0-127)</td>
<td>Pitch Bender MSB (0-127)</td>
</tr>
<tr valign="top">
<td>11100011= E3= 227</td>
<td>Chan 4 Pitch Bend Change</td>
<td>Pitch Bender LSB (0-127)</td>
<td>Pitch Bender MSB (0-127)</td>
</tr>
<tr valign="top">
<td>11100100= E4= 228</td>
<td>Chan 5 Pitch Bend Change</td>
<td>Pitch Bender LSB (0-127)</td>
<td>Pitch Bender MSB (0-127)</td>
</tr>
<tr valign="top">
<td>11100101= E5= 229</td>
<td>Chan 6 Pitch Bend Change</td>
<td>Pitch Bender LSB (0-127)</td>
<td>Pitch Bender MSB (0-127)</td>
</tr>
<tr valign="top">
<td>11100110= E6= 230</td>
<td>Chan 7 Pitch Bend Change</td>
<td>Pitch Bender LSB (0-127)</td>
<td>Pitch Bender MSB (0-127)</td>
</tr>
<tr valign="top">
<td>11100111= E7= 231</td>
<td>Chan 8 Pitch Bend Change</td>
<td>Pitch Bender LSB (0-127)</td>
<td>Pitch Bender MSB (0-127)</td>
</tr>
<tr valign="top">
<td>11101000= E8= 232</td>
<td>Chan 9 Pitch Bend Change</td>
<td>Pitch Bender LSB (0-127)</td>
<td>Pitch Bender MSB (0-127)</td>
</tr>
<tr valign="top">
<td>11101001= E9= 233</td>
<td>Chan 10 Pitch Bend Change</td>
<td>Pitch Bender LSB (0-127)</td>
<td>Pitch Bender MSB (0-127)</td>
</tr>
<tr valign="top">
<td>11101010= EA= 234</td>
<td>Chan 11 Pitch Bend Change</td>
<td>Pitch Bender LSB (0-127)</td>
<td>Pitch Bender MSB (0-127)</td>
</tr>
<tr valign="top">
<td>11101011= EB= 235</td>
<td>Chan 12 Pitch Bend Change</td>
<td>Pitch Bender LSB (0-127)</td>
<td>Pitch Bender MSB (0-127)</td>
</tr>
<tr valign="top">
<td>11101100= EC= 236</td>
<td>Chan 13 Pitch Bend Change</td>
<td>Pitch Bender LSB (0-127)</td>
<td>Pitch Bender MSB (0-127)</td>
</tr>
<tr valign="top">
<td>11101101= ED= 237</td>
<td>Chan 14 Pitch Bend Change</td>
<td>Pitch Bender LSB (0-127)</td>
<td>Pitch Bender MSB (0-127)</td>
</tr>
<tr valign="top">
<td>11101110= EE= 238</td>
<td>Chan 15 Pitch Bend Change</td>
<td>Pitch Bender LSB (0-127)</td>
<td>Pitch Bender MSB (0-127)</td>
</tr>
<tr valign="top">
<td>11101111= EF= 239</td>
<td>Chan 16 Pitch Bend Change</td>
<td>Pitch Bender LSB (0-127)</td>
<td>Pitch Bender MSB (0-127)</td>
</tr>
<tr valign="top">
<td>11110000= F0= 240</td>
<td>System Exclusive</td>
<td>**</td>
<td>**</td>
</tr>
<tr valign="top">
<td>11110001= F1= 241</td>
<td>MIDI Time Code Qtr. Frame</td>
<td>-see spec-</td>
<td>-see spec-</td>
</tr>
<tr valign="top">
<td>11110010= F2= 242</td>
<td>Song Position Pointer</td>
<td>LSB</td>
<td>MSB</td>
</tr>
<tr valign="top">
<td>11110011= F3= 243</td>
<td>Song Select (Song #)</td>
<td>(0-127)</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11110100= F4= 244</td>
<td>Undefined (Reserved)</td>
<td>—</td>
<td>—</td>
</tr>
<tr valign="top">
<td>11110101= F5= 245</td>
<td>Undefined (Reserved)</td>
<td>—</td>
<td>—</td>
</tr>
<tr valign="top">
<td>11110110= F6= 246</td>
<td>Tune request</td>
<td>none</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11110111= F7= 247</td>
<td>End of SysEx (EOX)</td>
<td>none</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11111000= F8= 248</td>
<td>Timing clock</td>
<td>none</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11111001= F9= 249</td>
<td>Undefined (Reserved)</td>
<td>—</td>
<td>—</td>
</tr>
<tr valign="top">
<td>11111010= FA= 250</td>
<td>Start</td>
<td>none</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11111011= FB= 251</td>
<td>Continue</td>
<td>none</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11111100= FC= 252</td>
<td>Stop</td>
<td>none</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11111101= FD= 253</td>
<td>Undefined (Reserved)</td>
<td>—</td>
<td>—</td>
</tr>
<tr valign="top">
<td>11111110= FE= 254</td>
<td>Active Sensing</td>
<td>none</td>
<td>none</td>
</tr>
<tr valign="top">
<td>11111111= FF= 255</td>
<td>System Reset</td>
<td>none</td>
<td>none</td>
</tr>
<tr valign="top">
<td colspan="4">** Note: System Exclusive (data dump) 2nd byte= Vendor ID (or Universal Exclusive) followed by more data bytes and ending with EOX.</td>
</tr>
</tbody>
</table>
